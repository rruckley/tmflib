//! tmf-gen : Code generator for tmflib
//!
//! Parses TMForum OpenAPI 3.x specification files and generates Rust model
//! code compatible with the `tmflib` crate. Exposed as a library so that it
//! can be invoked either from the bundled CLI (`main.rs`) or from tmflib's
//! `build.rs`.
//!
//! Generation pipeline:
//! 1. Parse the spec with `openapiv3`
//! 2. Convert schemas to JSON Schema (`schemars`) preserving `$ref` strings
//! 3. Strip `allOf` branches that reference shared (crate-level) base types
//! 4. Convert to Rust with `typify`
//! 5. Post-process the syntax tree: rewrite shared-type paths, remove shared
//!    definitions, re-insert shared bases as `#[serde(flatten)]` fields,
//!    inject `CLASS_PATH` consts and `HasId`/`HasDescription` derives for
//!    managed types, and add `Display`/`Deref` impls.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use openapiv3::OpenAPI;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use typify::{TypeSpace, TypeSpaceSettings};

/// Error type covering all failure modes of the generator.
#[derive(Debug)]
pub enum GenError {
    /// The spec file could not be read or written.
    Io(std::io::Error),
    /// The spec file could not be parsed as OpenAPI 3.x JSON.
    Json(serde_json::Error),
    /// The spec file could not be parsed as OpenAPI 3.x YAML.
    Yaml(serde_yaml::Error),
    /// typify failed to convert a schema into Rust types.
    Typify(typify::Error),
    /// The parsed spec contains no `components` section.
    NoComponents,
    /// A schema could not be converted to JSON Schema form.
    Conversion(serde_json::Error),
    /// Generated code could not be parsed back into a syntax tree.
    Syn(syn::Error),
}

impl fmt::Display for GenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenError::Io(e) => write!(f, "I/O error: {e}"),
            GenError::Json(e) => write!(f, "Could not parse spec as OpenAPI JSON: {e}"),
            GenError::Yaml(e) => write!(f, "Could not parse spec as OpenAPI YAML: {e}"),
            GenError::Typify(e) => write!(f, "Schema conversion failed: {e}"),
            GenError::NoComponents => write!(f, "Spec contains no components section"),
            GenError::Conversion(e) => write!(f, "Could not convert schema to JSON Schema: {e}"),
            GenError::Syn(e) => write!(f, "Generated code failed to parse: {e}"),
        }
    }
}

impl std::error::Error for GenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GenError::Io(e) => Some(e),
            GenError::Json(e) => Some(e),
            GenError::Yaml(e) => Some(e),
            GenError::Typify(e) => Some(e),
            GenError::NoComponents | GenError::Conversion(_) => None,
            GenError::Syn(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for GenError {
    fn from(e: std::io::Error) -> Self {
        GenError::Io(e)
    }
}

/// A schema type that is provided by the `tmflib` crate itself (e.g.
/// `Extensible`, `Entity`, `TimePeriod`) rather than generated per module.
#[derive(Debug, Clone)]
pub struct SharedType {
    /// Schema name as it appears in the OAS spec, e.g. `Extensible`.
    pub schema: String,
    /// Fully-qualified Rust path for the replacement type.
    pub path: String,
    /// Field name used when this type is flattened into a sub-type
    /// (from an `allOf` composition), e.g. `extensible`.
    pub field: String,
    /// Inlined field names contributed by this base which must be dropped
    /// from sub-types, e.g. `base_type`, `schema_location`, `type_`.
    pub drop_fields: Vec<String>,
}

impl SharedType {
    /// Define a shared type mapping with no dropped fields.
    pub fn new(schema: &str, path: &str, field: &str) -> Self {
        SharedType {
            schema: schema.to_string(),
            path: path.to_string(),
            field: field.to_string(),
            drop_fields: Vec::new(),
        }
    }

    /// Record the fields contributed by this base that are dropped from
    /// sub-types in favour of the flattened shared struct.
    pub fn with_dropped(mut self, fields: &[&str]) -> Self {
        self.drop_fields = fields.iter().map(|f| f.to_string()).collect();
        self
    }
}

/// A schema type that is "managed" by tmflib : it receives a `CLASS_PATH`
/// const and `HasId`/`HasDescription` derives so it can participate in
/// TMF URL generation.
#[derive(Debug, Clone)]
pub struct ManagedType {
    /// Schema name as it appears in the OAS spec, e.g. `PerformanceMeasurement`.
    pub schema: String,
    /// Value for the `CLASS_PATH` const, e.g. `measurement`.
    pub class_path: String,
    /// Generate an `IsAddressable` impl for this type.
    pub addressable: bool,
}

impl ManagedType {
    /// Define a managed type with a class path and no addressability.
    pub fn new(schema: &str, class_path: &str) -> Self {
        ManagedType {
            schema: schema.to_string(),
            class_path: class_path.to_string(),
            addressable: false,
        }
    }

    /// Mark this managed type as addressable.
    pub fn addressable(mut self) -> Self {
        self.addressable = true;
        self
    }
}

/// Per-module configuration applied during generation.
#[derive(Debug, Clone)]
pub struct ModuleConfig {
    /// TMF module number, e.g. `tmf628`.
    pub tmf: String,
    /// OAS spec file name (relative to the `open_api/` folder) this module
    /// is generated from, e.g. `TMF628_Performance-v5.0.0.oas.yaml`.
    pub spec_file: Option<String>,
    /// Value for the module-level `TMF_MODULE` const, e.g.
    /// `performanceManagement`. Required when managed types are present.
    pub tmf_module: Option<String>,
    /// Schema types supplied by the crate instead of generated.
    pub shared_types: Vec<SharedType>,
    /// Schema types that receive `CLASS_PATH` and trait derives.
    pub managed_types: Vec<ManagedType>,
}

impl ModuleConfig {
    /// Create a configuration for a TMF module with the default set of
    /// shared types and no managed types.
    pub fn new(tmf: impl Into<String>) -> Self {
        ModuleConfig {
            tmf: tmf.into(),
            spec_file: None,
            tmf_module: None,
            shared_types: default_shared_types(),
            managed_types: Vec::new(),
        }
    }

    /// Set the OAS spec file name for this module.
    pub fn with_spec_file(mut self, spec_file: impl Into<String>) -> Self {
        self.spec_file = Some(spec_file.into());
        self
    }

    /// Set the `TMF_MODULE` const value.
    pub fn with_tmf_module(mut self, tmf_module: impl Into<String>) -> Self {
        self.tmf_module = Some(tmf_module.into());
        self
    }

    /// Add or replace a shared type mapping.
    pub fn with_shared(mut self, shared: SharedType) -> Self {
        self.shared_types.retain(|s| s.schema != shared.schema);
        self.shared_types.push(shared);
        self
    }

    /// Add a managed type.
    pub fn with_managed(mut self, managed: ManagedType) -> Self {
        self.managed_types.push(managed);
        self
    }
}

/// Default shared-type table covering the base types that tmflib provides
/// in `common/` and at the crate root.
pub fn default_shared_types() -> Vec<SharedType> {
    vec![
        SharedType::new(
            "Extensible",
            "crate::common::extensible::Extensible",
            "extensible",
        )
        .with_dropped(&["base_type", "schema_location", "type_"]),
        SharedType::new(
            "ExtensibleFvo",
            "crate::common::extensible::ExtensibleFvo",
            "extensible",
        )
        .with_dropped(&["base_type", "schema_location", "type_"]),
        SharedType::new("Entity", "crate::common::entity::Entity", "entity").with_dropped(&[
            "base_type",
            "schema_location",
            "type_",
        ]),
        SharedType::new(
            "EntityRef",
            "crate::common::entity::EntityRef",
            "extensible",
        ),
        SharedType::new(
            "Addressable",
            "crate::common::addressable::Addressable",
            "addressable",
        )
        .with_dropped(&["base_type", "schema_location", "type_"]),
        SharedType::new("Note", "crate::common::note::Note", "note"),
        SharedType::new(
            "PlaceRef",
            "crate::common::related_place::PlaceRef",
            "place",
        ),
        SharedType::new(
            "ExternalIdentifier",
            "crate::common::external_identifier::ExternalIdentifier",
            "external_identifier",
        ),
        SharedType::new("TimePeriod", "crate::TimePeriod", "time_period"),
        // Synthetic schema produced by the date-time rewrite; mapped onto
        // tmflib's string-based timestamp alias.
        SharedType::new("TimeStamp", "crate::TimeStamp", "timestamp"),
    ]
}

/// Load a spec from a JSON or YAML file (chosen by extension), detecting
/// whether it is OpenAPI 3.x or Swagger 2.0 and normalizing to [`Spec`].
pub fn load_spec(path: &Path) -> Result<Spec, GenError> {
    let contents = fs::read_to_string(path)?;
    let value: serde_json::Value = match path.extension().and_then(|e| e.to_str()) {
        Some("yaml") | Some("yml") => serde_yaml::from_str(&contents).map_err(GenError::Yaml)?,
        _ => serde_json::from_str(&contents).map_err(GenError::Json)?,
    };
    if value.get("swagger").and_then(serde_json::Value::as_str) == Some("2.0") {
        return swagger_definitions(&value);
    }
    let openapi: OpenAPI = serde_json::from_value(value).map_err(GenError::Json)?;
    let components = openapi.components.as_ref().ok_or(GenError::NoComponents)?;
    Ok(Spec {
        title: openapi.info.title.clone(),
        version: openapi.info.version.clone(),
        description: openapi.info.description.clone(),
        schemas: convert_schemas(&components.schemas)?,
    })
}

/// Apache 2.0 license header applied to generated files, matching tmflib.
const LICENSE_HEADER: &str = "\
// Copyright [2026] [Ryan Ruckley]
//
// Licensed under the Apache License, Version 2.0 (the \"License\");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an \"AS IS\" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.";

/// Extract the final path segment of a `$ref` string,
/// e.g. `#/components/schemas/Extensible` -> `Extensible`.
fn ref_tail(reference: &str) -> &str {
    reference.rsplit('/').next().unwrap_or(reference)
}

/// Return the schema name a (schemars) schema refers to, if it is a `$ref`.
fn ref_name(schema: &schemars::schema::Schema) -> Option<String> {
    if let schemars::schema::Schema::Object(obj) = schema {
        if let Some(reference) = &obj.reference {
            return Some(ref_tail(reference).to_string());
        }
    }
    None
}

/// Format-normalized view of a spec's schema definitions.
///
/// Both OpenAPI 3.x (`components.schemas`) and Swagger 2.0 (`definitions`)
/// are normalized into this shape so the generation pipeline is
/// format-agnostic. `$ref` strings keep their original form; typify
/// resolves them by the final path segment, which matches the schema names
/// supplied to [`TypeSpace::add_ref_types`] in both formats.
#[derive(Debug, Clone)]
pub struct Spec {
    /// Title from the spec's `info` section.
    pub title: String,
    /// Version from the spec's `info` section.
    pub version: String,
    /// Description from the spec's `info` section, if present.
    pub description: Option<String>,
    /// Named schema definitions.
    pub schemas: BTreeMap<String, schemars::schema::Schema>,
}

/// Convert openapiv3 schemas into the schemars JSON Schema types that
/// typify consumes. The round-trip through `serde_json::Value` preserves
/// `$ref` strings (`#/components/schemas/Name`), which typify resolves
/// against the names supplied to [`TypeSpace::add_ref_types`].
fn convert_schemas(
    schemas: &indexmap::IndexMap<String, openapiv3::ReferenceOr<openapiv3::Schema>>,
) -> Result<BTreeMap<String, schemars::schema::Schema>, GenError> {
    schemas
        .iter()
        .map(|(name, reference_or)| {
            let value = serde_json::to_value(reference_or).map_err(GenError::Conversion)?;
            let schema = serde_json::from_value::<schemars::schema::Schema>(value)
                .map_err(GenError::Conversion)?;
            Ok((name.clone(), schema))
        })
        .collect()
}

/// Extract the schema definitions from a Swagger 2.0 document.
///
/// Only the `definitions` section is needed for model generation; paths,
/// parameters and responses are ignored. The 2.0 schema object syntax is a
/// subset of the JSON Schema dialect that `schemars` parses, so the
/// definitions are converted directly without an intermediate model.
fn swagger_definitions(value: &serde_json::Value) -> Result<Spec, GenError> {
    let info = value.get("info").ok_or(GenError::NoComponents)?;
    let title = info
        .get("title")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("Swagger 2.0 spec")
        .to_string();
    let version = info
        .get("version")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let description = info
        .get("description")
        .and_then(serde_json::Value::as_str)
        .map(str::to_string);
    let definitions = value
        .get("definitions")
        .and_then(serde_json::Value::as_object)
        .ok_or(GenError::NoComponents)?;
    let schemas = definitions
        .iter()
        .map(|(name, schema)| {
            let schema = serde_json::from_value::<schemars::schema::Schema>(schema.clone())
                .map_err(GenError::Conversion)?;
            Ok((name.clone(), schema))
        })
        .collect::<Result<BTreeMap<_, _>, GenError>>()?;
    Ok(Spec {
        title,
        version,
        description,
        schemas,
    })
}

/// Recursively replace string schemas with `format: "date-time"` (or
/// `"date"`) by references to a synthetic named schema so that typify does
/// not emit `chrono` types, which tmflib avoids (see `crate::TimeStamp`).
fn rewrite_datetime_refs(schema: &mut schemars::schema::Schema, changed: &mut bool) {
    let schemars::schema::Schema::Object(obj) = schema else {
        return;
    };
    let is_string_type = obj.instance_type.as_ref().is_some_and(|t| match t {
        schemars::schema::SingleOrVec::Single(it) => **it == schemars::schema::InstanceType::String,
        schemars::schema::SingleOrVec::Vec(v) => {
            v.contains(&schemars::schema::InstanceType::String)
        }
    });
    let is_temporal_format = obj
        .format
        .as_deref()
        .map(|f| f == "date-time" || f == "date")
        .unwrap_or(false);
    let is_temporal_string = is_string_type && is_temporal_format;
    if is_temporal_string {
        *obj = schemars::schema::SchemaObject::default();
        obj.reference = Some("#/components/schemas/TimeStamp".to_string());
        *changed = true;
        return;
    }
    // Recurse into subschemas.
    if let Some(subschemas) = &mut obj.subschemas {
        for branch in subschemas
            .all_of
            .iter_mut()
            .flatten()
            .chain(subschemas.any_of.iter_mut().flatten())
            .chain(subschemas.one_of.iter_mut().flatten())
        {
            rewrite_datetime_refs(branch, changed);
        }
        if let Some(not) = &mut subschemas.not {
            rewrite_datetime_refs(not.as_mut(), changed);
        }
    }
    // Recurse into object properties.
    if let Some(object) = &mut obj.object {
        for property in object.properties.values_mut() {
            rewrite_datetime_refs(property, changed);
        }
        if let Some(additional) = &mut object.additional_properties {
            rewrite_datetime_refs(additional.as_mut(), changed);
        }
    }
    // Recurse into array items.
    if let Some(array) = &mut obj.array {
        if let Some(item_schema) = &mut array.items {
            match item_schema {
                schemars::schema::SingleOrVec::Single(s) => {
                    rewrite_datetime_refs(s.as_mut(), changed);
                }
                schemars::schema::SingleOrVec::Vec(v) => {
                    for s in v.iter_mut() {
                        rewrite_datetime_refs(s, changed);
                    }
                }
            }
        }
    }
}

/// Apply all spec-recovery preprocessing steps to a raw schema map:
/// synthesize missing `$ref` targets, drop numeric defaults, and rewrite
/// date-time/date strings onto the synthetic `TimeStamp` schema.
/// Returns warnings describing what was changed.
#[doc(hidden)]
pub fn preprocess_schemas(schemas: &mut BTreeMap<String, schemars::schema::Schema>) -> Vec<String> {
    let mut warnings = Vec::new();
    let placeholders = synthesize_missing_schemas(schemas);
    if !placeholders.is_empty() {
        warnings.push(format!(
            "synthesized empty placeholder schemas for missing $ref targets: {}",
            placeholders.join(", ")
        ));
    }
    let dropped_defaults = strip_all_defaults(schemas);
    if !dropped_defaults.is_empty() {
        warnings.push(format!(
            "dropped schema default values from: {}",
            dropped_defaults.join(", ")
        ));
    }
    let mut datetime_seen = false;
    for schema in schemas.values_mut() {
        rewrite_datetime_refs(schema, &mut datetime_seen);
    }
    if datetime_seen {
        schemas.insert(
            "TimeStamp".to_string(),
            serde_json::from_str("{\"type\":\"string\"}").expect("static schema"),
        );
    }
    warnings
}

/// Test/diagnostic wrapper for [`strip_shared_bases`] using a shared-type
/// list (as found in [`ModuleConfig::shared_types`]).
#[doc(hidden)]
pub fn strip_shared_bases_for_list(
    schemas: &mut BTreeMap<String, schemars::schema::Schema>,
    shared: &[SharedType],
) -> BTreeMap<String, Vec<String>> {
    let by_name: BTreeMap<String, &SharedType> =
        shared.iter().map(|s| (s.schema.clone(), s)).collect();
    strip_shared_bases(schemas, &by_name)
}

/// Recursively visit every schema object in a schema tree.
fn visit_schema_objects(
    schema: &mut schemars::schema::Schema,
    f: &mut impl FnMut(&mut schemars::schema::SchemaObject),
) {
    let schemars::schema::Schema::Object(obj) = schema else {
        return;
    };
    f(obj);
    if let Some(subschemas) = &mut obj.subschemas {
        for branch in subschemas
            .all_of
            .iter_mut()
            .flatten()
            .chain(subschemas.any_of.iter_mut().flatten())
            .chain(subschemas.one_of.iter_mut().flatten())
        {
            visit_schema_objects(branch, f);
        }
        if let Some(not) = &mut subschemas.not {
            visit_schema_objects(not.as_mut(), f);
        }
    }
    if let Some(object) = &mut obj.object {
        for property in object.properties.values_mut() {
            visit_schema_objects(property, f);
        }
        if let Some(additional) = &mut object.additional_properties {
            visit_schema_objects(additional.as_mut(), f);
        }
    }
    if let Some(array) = &mut obj.array {
        if let Some(item_schema) = &mut array.items {
            match item_schema {
                schemars::schema::SingleOrVec::Single(s) => {
                    visit_schema_objects(s.as_mut(), f);
                }
                schemars::schema::SingleOrVec::Vec(v) => {
                    for s in v.iter_mut() {
                        visit_schema_objects(s, f);
                    }
                }
            }
        }
    }
}

/// Synthesize empty object schemas for `$ref` targets that the spec
/// references but never defines (a recurring quality issue in TMF specs).
/// Returns the names of the placeholders that were added.
fn synthesize_missing_schemas(
    schemas: &mut BTreeMap<String, schemars::schema::Schema>,
) -> Vec<String> {
    let mut referenced: BTreeSet<String> = BTreeSet::new();
    for schema in schemas.values_mut() {
        visit_schema_objects(schema, &mut |obj| {
            if let Some(reference) = &obj.reference {
                referenced.insert(ref_tail(reference).to_string());
            }
        });
    }
    let missing: Vec<String> = referenced
        .iter()
        .filter(|name| !schemas.contains_key(*name))
        .cloned()
        .collect();
    for name in &missing {
        let schema: schemars::schema::Schema =
            serde_json::from_str("{\"type\":\"object\"}").expect("static schema");
        schemas.insert(name.clone(), schema);
    }
    missing
}

/// Remove all `default` values from schemas. TMF specs contain several
/// invalid defaults (e.g. string `"false"` on boolean fields, integers
/// outside format bounds) that typify rejects; tmflib does not use schema
/// defaults, so they are simply dropped. Returns the affected schema names.
fn strip_all_defaults(schemas: &mut BTreeMap<String, schemars::schema::Schema>) -> Vec<String> {
    let mut dropped: Vec<String> = Vec::new();
    for (name, schema) in schemas.iter_mut() {
        let mut hit = false;
        visit_schema_objects(schema, &mut |obj| {
            if let Some(metadata) = &mut obj.metadata {
                if metadata.default.is_some() {
                    metadata.default = None;
                    hit = true;
                }
            }
        });
        if hit {
            dropped.push(name.clone());
        }
    }
    dropped
}

/// Strip `allOf` branches that reference shared types and record which
/// bases were removed for each schema so the post-processor can re-insert
/// them as flattened fields.
fn strip_shared_bases(
    schemas: &mut BTreeMap<String, schemars::schema::Schema>,
    shared: &BTreeMap<String, &SharedType>,
) -> BTreeMap<String, Vec<String>> {
    let mut substitutions: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (name, schema) in schemas.iter_mut() {
        let schemars::schema::Schema::Object(obj) = schema else {
            continue;
        };
        let Some(subschemas) = &mut obj.subschemas else {
            continue;
        };
        let Some(all_of) = &mut subschemas.all_of else {
            continue;
        };
        let mut kept = Vec::new();
        let mut bases = Vec::new();
        for branch in all_of.drain(..) {
            match ref_name(&branch) {
                Some(r) if shared.contains_key(&r) => bases.push(r),
                _ => kept.push(branch),
            }
        }
        if kept.is_empty() {
            subschemas.all_of = None;
        } else {
            subschemas.all_of = Some(kept);
        }
        if !bases.is_empty() {
            substitutions.insert(name.clone(), bases);
        }
    }
    substitutions
}

/// Rewrites type paths that refer to locally-generated shared types into
/// their fully-qualified crate paths.
struct SharedPathRewriter<'a> {
    map: &'a BTreeMap<String, String>,
}

impl VisitMut for SharedPathRewriter<'_> {
    fn visit_type_path_mut(&mut self, path: &mut syn::TypePath) {
        if path.qself.is_none() {
            if let Some(last) = path.path.segments.last() {
                if let Some(replacement) = self.map.get(&last.ident.to_string()) {
                    if let Ok(new_path) = syn::parse_str::<syn::TypePath>(replacement) {
                        *path = new_path;
                    }
                    // Do not recurse into the replaced path: its last
                    // segment deliberately matches the map key.
                    return;
                }
            }
        }
        syn::visit_mut::visit_type_path_mut(self, path);
    }
}

/// Truncates doc comments at the embedded JSON schema dump that typify
/// emits (`<details><summary>JSON schema ...`). typify emits doc comments
/// line-by-line as separate `#[doc]` attributes, so everything from the
/// marker attribute onwards must be removed.
fn strip_schema_docs(attrs: &mut Vec<syn::Attribute>) {
    let mut cut_at: Option<usize> = None;
    for (idx, attr) in attrs.iter().enumerate() {
        if !attr.path().is_ident("doc") {
            continue;
        }
        let value = match &attr.meta {
            syn::Meta::NameValue(nv) => match &nv.value {
                syn::Expr::Lit(lit) => match &lit.lit {
                    syn::Lit::Str(s) => s.value(),
                    _ => continue,
                },
                _ => continue,
            },
            _ => continue,
        };
        if value.contains("<details><summary>JSON schema") {
            cut_at = Some(idx);
            break;
        }
    }
    if let Some(idx) = cut_at {
        // Truncate the marker attribute's text, then drop every doc
        // attribute that followed it (the JSON dump lines).
        if let syn::Meta::NameValue(nv) = &mut attrs[idx].meta {
            if let syn::Expr::Lit(lit) = &mut nv.value {
                if let syn::Lit::Str(s) = &mut lit.lit {
                    let value = s.value();
                    let trimmed = value[..value.find("<details").unwrap()]
                        .trim_end()
                        .to_string();
                    *s = syn::LitStr::new(&trimmed, s.span());
                }
            }
        }
        // Remove the doc attributes that followed the marker (the JSON
        // dump lines) while preserving any non-doc attributes, e.g.
        // `#[derive(...)]`, that come after the doc block.
        let mut idx = idx + 1;
        while idx < attrs.len() {
            if attrs[idx].path().is_ident("doc") {
                attrs.remove(idx);
            } else {
                idx += 1;
            }
        }
    }
}

/// Visitor applying [`strip_schema_docs`] to every attributed node.
struct DocStripper;

impl VisitMut for DocStripper {
    fn visit_item_mut(&mut self, item: &mut syn::Item) {
        let attrs = match item {
            syn::Item::Struct(s) => &mut s.attrs,
            syn::Item::Enum(e) => &mut e.attrs,
            syn::Item::Fn(f) => &mut f.attrs,
            syn::Item::Impl(i) => &mut i.attrs,
            syn::Item::Mod(m) => &mut m.attrs,
            syn::Item::Type(t) => &mut t.attrs,
            syn::Item::Const(c) => &mut c.attrs,
            syn::Item::Use(u) => &mut u.attrs,
            _ => {
                syn::visit_mut::visit_item_mut(self, item);
                return;
            }
        };
        strip_schema_docs(attrs);
        syn::visit_mut::visit_item_mut(self, item);
    }

    fn visit_field_mut(&mut self, field: &mut syn::Field) {
        strip_schema_docs(&mut field.attrs);
        syn::visit_mut::visit_field_mut(self, field);
    }

    fn visit_variant_mut(&mut self, variant: &mut syn::Variant) {
        strip_schema_docs(&mut variant.attrs);
        syn::visit_mut::visit_variant_mut(self, variant);
    }
}

/// Locate the struct item for a given type name.
fn find_struct<'a>(items: &'a [syn::Item], name: &str) -> Option<&'a syn::ItemStruct> {
    items.iter().find_map(|item| match item {
        syn::Item::Struct(s) if s.ident == name => Some(s),
        _ => None,
    })
}

/// Identify the ident a top-level item defines, if any.
fn defined_ident(item: &syn::Item) -> Option<String> {
    match item {
        syn::Item::Struct(s) => Some(s.ident.to_string()),
        syn::Item::Enum(e) => Some(e.ident.to_string()),
        syn::Item::Type(t) => Some(t.ident.to_string()),
        _ => None,
    }
}

/// Identify the self type of an impl block, if it is a simple path.
fn impl_self_ident(item: &syn::Item) -> Option<String> {
    if let syn::Item::Impl(imp) = item {
        if let syn::Type::Path(tp) = imp.self_ty.as_ref() {
            if tp.qself.is_none() {
                if let Some(last) = tp.path.segments.last() {
                    return Some(last.ident.to_string());
                }
            }
        }
    }
    None
}

/// Collect the head idents of every type path in a type, including
/// generic arguments, e.g. `Option<Vec<Note>>` -> `Option`, `Vec`, `Note`.
fn type_head_idents(ty: &syn::Type, out: &mut Vec<String>) {
    if let syn::Type::Path(tp) = ty {
        for segment in &tp.path.segments {
            out.push(segment.ident.to_string());
            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                for arg in args.args.iter() {
                    if let syn::GenericArgument::Type(inner) = arg {
                        type_head_idents(inner, out);
                    }
                }
            }
        }
    }
}

/// Collect the head idents of every type path in a path, including
/// generic arguments.
fn path_head_idents(path: &syn::Path, out: &mut Vec<String>) {
    for segment in &path.segments {
        out.push(segment.ident.to_string());
        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
            for arg in args.args.iter() {
                if let syn::GenericArgument::Type(inner) = arg {
                    type_head_idents(inner, out);
                }
            }
        }
    }
}

/// True if an impl block's trait or self type mentions one of the given
/// idents, e.g. `impl From<Entity> for serde_json::Map<...>` for a shared
/// `Entity`.
fn impl_refs_shared(item: &syn::Item, shared: &std::collections::BTreeSet<String>) -> bool {
    let syn::Item::Impl(imp) = item else {
        return false;
    };
    let mut idents = Vec::new();
    if let Some((_, path, _)) = &imp.trait_ {
        path_head_idents(path, &mut idents);
    }
    type_head_idents(imp.self_ty.as_ref(), &mut idents);
    idents.iter().any(|ident| shared.contains(ident))
}

/// Generate the model source for a loaded spec.
pub fn generate(spec: &Spec, config: &ModuleConfig) -> Result<String, GenError> {
    let shared_by_name: BTreeMap<String, &SharedType> = config
        .shared_types
        .iter()
        .map(|s| (s.schema.clone(), s))
        .collect();

    let mut schemas = spec.schemas.clone();

    // Recover from known TMF spec-quality issues and record what was done.
    let warnings = preprocess_schemas(&mut schemas);

    let substitutions = strip_shared_bases(&mut schemas, &shared_by_name);

    // typify emits `#[derive(Debug, Clone, Deserialize, Serialize)]` by default.
    let mut settings = TypeSpaceSettings::default();
    // Map typify's built-in date-time type onto tmflib's string-based
    // TimeStamp alias (tmflib deliberately avoids chrono serde support).
    settings.with_replacement(
        "chrono::DateTime<chrono::offset::Utc>",
        "crate::TimeStamp",
        // No FromStr/Display/Default impls: `crate::TimeStamp` is an alias
        // for `String`, a foreign type.
        std::iter::empty(),
    );
    let mut type_space = TypeSpace::new(&settings);
    type_space
        .add_ref_types(schemas)
        .map_err(GenError::Typify)?;

    let stream = type_space.to_stream();
    let mut file = syn::parse2::<syn::File>(stream).map_err(GenError::Syn)?;

    // 1. Trim typify's embedded JSON schema dumps from doc comments.
    DocStripper.visit_file_mut(&mut file);

    // 2. Rewrite type paths referring to shared types to crate paths.
    let path_map: BTreeMap<String, String> = config
        .shared_types
        .iter()
        .map(|s| (s.schema.clone(), s.path.clone()))
        .collect();
    SharedPathRewriter { map: &path_map }.visit_file_mut(&mut file);

    // 3. Remove the shared type definitions (structs, enums and impls).
    let shared_names: std::collections::BTreeSet<String> = config
        .shared_types
        .iter()
        .map(|s| s.schema.clone())
        .collect();
    file.items.retain(|item| {
        let defines = defined_ident(item)
            .map(|ident| shared_names.contains(&ident))
            .unwrap_or(false);
        let impls = impl_self_ident(item)
            .map(|ident| shared_names.contains(&ident))
            .unwrap_or(false);
        // Also drop impls that mention shared types in their headers, e.g.
        // typify's `impl From<SharedType> for serde_json::Map<...>` which
        // assumes a transparent newtype layout.
        let shared_impls = impl_refs_shared(item, &shared_names);
        !(defines || impls || shared_impls)
    });

    // 4. Re-insert shared bases as flattened fields on composed types.
    // Structs modified here (and managed types below) get their typify
    // Default impls replaced with a derive, since the typify impls are
    // struct literals that would miss the injected fields.
    let mut modified: Vec<String> = Vec::new();
    for (type_name, bases) in &substitutions {
        modified.push(type_name.clone());
        let Some(item) = find_struct(&file.items, type_name) else {
            continue;
        };
        let struct_item = item.clone();
        let syn::Fields::Named(fields) = &struct_item.fields else {
            continue;
        };
        let mut new_fields: syn::punctuated::Punctuated<syn::Field, syn::token::Comma> =
            syn::punctuated::Punctuated::new();
        for base in bases {
            let Some(shared) = shared_by_name.get(base) else {
                continue;
            };
            let path: syn::Type = syn::parse_str(&shared.path).map_err(GenError::Syn)?;
            let field_ident = syn::Ident::new(&shared.field, proc_macro2::Span::call_site());
            let field: syn::Field = syn::parse_quote! {
                #[serde(flatten)]
                pub #field_ident: #path
            };
            new_fields.push(field);
        }
        for field in fields.named.clone() {
            // Drop fields that came from the (now flattened) shared base.
            let drop = bases.iter().any(|base| {
                shared_by_name
                    .get(base)
                    .map(|s| {
                        field
                            .ident
                            .as_ref()
                            .map(|i| s.drop_fields.iter().any(|d| *i == d.as_str()))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            });
            if !drop {
                new_fields.push(field);
            }
        }
        // Splice the updated fields back into the file.
        let ident = struct_item.ident.clone();
        let attrs = struct_item.attrs.clone();
        let vis = struct_item.vis.clone();
        let new_struct: syn::ItemStruct = syn::parse_quote! {
            #(#attrs)*
            #vis struct #ident {
                #new_fields
            }
        };
        let pos = file
            .items
            .iter()
            .position(|i| matches!(i, syn::Item::Struct(s) if s.ident == ident))
            .expect("struct was just found");
        file.items[pos] = syn::Item::Struct(new_struct);
    }

    // 5. Managed types : CLASS_PATH consts and trait derives.
    let has_managed = !config.managed_types.is_empty();
    for managed in &config.managed_types {
        modified.push(managed.schema.clone());
        let Some(item) = find_struct(&file.items, &managed.schema) else {
            continue;
        };
        let struct_item = item.clone();
        let ident = struct_item.ident.clone();
        // Insert CLASS_PATH const before the struct.
        let managed_class_path = &managed.class_path;
        let class_const: syn::Item = syn::parse_quote! {
            /// Path to this class
            pub const CLASS_PATH: &str = #managed_class_path;
        };
        let mut new_struct = struct_item.clone();
        // The HasId derive needs id/href fields on the struct itself. When
        // they were contributed by a stripped shared base, re-add them as
        // own fields (matching the hand-maintained tmflib code).
        let original_fields: Vec<String> = struct_item
            .fields
            .iter()
            .filter_map(|f| f.ident.as_ref().map(|i| i.to_string()))
            .collect();
        if let syn::Fields::Named(fields) = &mut new_struct.fields {
            if !original_fields.contains(&"id".to_string()) {
                fields.named.insert(
                    1.min(fields.named.len()),
                    syn::parse_quote! {
                        /// Unique identifier of the object
                        pub id: ::std::option::Option<::std::string::String>
                    },
                );
            }
            if !original_fields.contains(&"href".to_string()) {
                fields.named.insert(
                    2.min(fields.named.len()),
                    syn::parse_quote! {
                        /// Hyperlink to the object
                        pub href: ::std::option::Option<::std::string::String>
                    },
                );
            }
        }
        // Derives: HasId requires id+href fields; HasDescription requires a
        // description field.
        let field_names: Vec<String> = new_struct
            .fields
            .iter()
            .filter_map(|f| f.ident.as_ref().map(|i| i.to_string()))
            .collect();
        if field_names.contains(&"id".to_string()) && field_names.contains(&"href".to_string()) {
            new_struct
                .attrs
                .push(syn::parse_quote!(#[derive(tmflib_derive::HasId)]));
        }
        if field_names.contains(&"description".to_string()) {
            new_struct
                .attrs
                .push(syn::parse_quote!(#[derive(tmflib_derive::HasDescription)]));
        }
        let pos = file
            .items
            .iter()
            .position(|i| matches!(i, syn::Item::Struct(s) if s.ident == ident))
            .expect("struct was just found");
        file.items[pos] = syn::Item::Struct(new_struct);
        file.items.insert(pos, class_const);
        // IsAddressable impl (appended at the end).
        if managed.addressable {
            let addressable_impl: syn::Item = syn::parse_quote! {
                impl IsAddressable for #ident {
                    fn get_objects() -> Vec<&'static str> {
                        get_objects()
                    }
                }
            };
            file.items.push(addressable_impl);
        }
    }

    // 5b. Replace typify's Default impls for modified structs with derives:
    // the typify impls are struct literals that would miss the injected
    // fields.
    let modified_set: std::collections::BTreeSet<String> = modified.into_iter().collect();
    file.items.retain(|item| {
        if let syn::Item::Impl(imp) = item {
            let is_default_trait = imp
                .trait_
                .as_ref()
                .and_then(|(_, path, _)| path.segments.last())
                .map(|s| s.ident == "Default")
                .unwrap_or(false);
            if is_default_trait {
                if let syn::Type::Path(tp) = imp.self_ty.as_ref() {
                    if let Some(last) = tp.path.segments.last() {
                        return !modified_set.contains(&last.ident.to_string());
                    }
                }
            }
        }
        true
    });
    for name in &modified_set {
        if let Some(s) = file.items.iter_mut().find_map(|item| match item {
            syn::Item::Struct(s) if s.ident == name => Some(s),
            _ => None,
        }) {
            s.attrs.push(syn::parse_quote!(#[derive(Default)]));
        }
    }

    // 6. Display for all structs; Deref/DerefMut for structs with a
    //    flattened shared base.
    let mut extra_impls: Vec<syn::Item> = Vec::new();
    for item in file.items.iter() {
        let syn::Item::Struct(s) = item else {
            continue;
        };
        let name = &s.ident;
        extra_impls.push(syn::parse_quote! {
            impl ::std::fmt::Display for #name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
                    write!(f, "{}", ::serde_json::to_string(self).unwrap())
                }
            }
        });
        if let syn::Fields::Named(fields) = &s.fields {
            if let Some(first) = fields.named.first() {
                let is_flatten = first.attrs.iter().any(|a| {
                    a.path().is_ident("serde")
                        && a.to_token_stream().to_string().contains("flatten")
                });
                if is_flatten {
                    if let (Some(field_ident), ty) = (&first.ident, &first.ty) {
                        extra_impls.push(syn::parse_quote! {
                            impl ::std::ops::Deref for #name {
                                type Target = #ty;
                                fn deref(&self) -> &Self::Target {
                                    &self.#field_ident
                                }
                            }
                        });
                        extra_impls.push(syn::parse_quote! {
                            impl ::std::ops::DerefMut for #name {
                                fn deref_mut(&mut self) -> &mut Self::Target {
                                    &mut self.#field_ident
                                }
                            }
                        });
                    }
                }
            }
        }
    }
    file.items.extend(extra_impls);

    // 7. Module prelude : TMF_MODULE const and required use statements.
    let mut prelude: Vec<syn::Item> = Vec::new();
    if has_managed {
        let tmf_module = config
            .tmf_module
            .clone()
            .unwrap_or_else(|| config.tmf.clone());
        prelude.push(syn::parse_quote! {
            /// TMF module path component used for HREF generation
            const TMF_MODULE: &str = #tmf_module;
        });
        prelude.push(syn::parse_quote! {
            use crate::TMF_VERSION;
        });
        prelude.push(syn::parse_quote! {
            use crate::{HasDescription, HasId};
        });
    }
    if config.managed_types.iter().any(|m| m.addressable) {
        let class_paths: Vec<String> = config
            .managed_types
            .iter()
            .filter(|m| m.addressable)
            .map(|m| m.class_path.clone())
            .collect();
        prelude.push(syn::parse_quote! {
            use crate::IsAddressable;
        });
        prelude.push(syn::parse_quote! {
            /// Classes that are addressable in this module
            fn get_objects() -> Vec<&'static str> {
                vec![#(#class_paths),*]
            }
        });
    }
    let mut items = prelude;
    items.extend(file.items);
    file.items = items;

    let body = prettyplease::unparse(&file);

    // Generated files are committed as plain modules in tmflib's `src/`
    // tree (regenerated on demand via `tmf-gen --all`), so inner doc
    // comments and inner attributes are used.
    let mut output = String::new();
    output.push_str(LICENSE_HEADER);
    output.push_str("\n\n");
    output.push_str("// Generated code: lint allowances applied module-wide.\n");
    output.push_str("#![allow(missing_docs)]\n");
    output.push_str("#![allow(clippy::all)]\n");
    output.push_str("#![allow(clippy::nursery)]\n");
    output.push_str("\n");
    output.push_str(&format!("//! {} - Generated module\n", config.tmf));
    output.push_str(&format!("//! Source: {} v{}\n", spec.title, spec.version));
    if let Some(description) = &spec.description {
        for line in description.lines() {
            output.push_str(&format!("//! {line}\n"));
        }
    }
    output.push_str("//!\n");
    output.push_str("//! This module is generated by `tmf-gen`. Do not edit by hand:\n");
    output.push_str("//! re-run `cargo run -p tmf-gen -- --all --output src` after changing\n");
    output.push_str("//! the OAS spec or the generator.\n");
    for warning in &warnings {
        output.push_str(&format!("// WARNING: {warning}\n"));
    }
    output.push_str(&body);

    Ok(output)
}

/// Convenience wrapper: load a spec from disk and generate its source.
pub fn generate_spec_file(spec_path: &Path, config: &ModuleConfig) -> Result<String, GenError> {
    let openapi = load_spec(spec_path)?;
    generate(&openapi, config)
}

/// Like [`generate`], but also returns diagnostics about spec issues that
/// the generator had to work around (mirrored as `// WARNING:` comments in
/// the generated source).
pub fn generate_with_diagnostics(
    spec: &Spec,
    config: &ModuleConfig,
) -> Result<(String, Vec<String>), GenError> {
    let code = generate(spec, config)?;
    let warnings: Vec<String> = code
        .lines()
        .filter_map(|line| line.strip_prefix("// WARNING: "))
        .map(str::to_string)
        .collect();
    Ok((code, warnings))
}

/// Write generated source to `<out_dir>/<tmf>.rs`, returning the file path.
pub fn write_output(
    out_dir: &Path,
    config: &ModuleConfig,
    code: &str,
) -> Result<PathBuf, GenError> {
    fs::create_dir_all(out_dir)?;
    let path = out_dir.join(format!("{}.rs", config.tmf));
    fs::write(&path, code)?;
    Ok(path)
}

/// TMF modules that have generator configurations and can be produced at
/// build time. Keep in sync with [`config_for_feature`].
pub const SUPPORTED_MODULES: &[&str] = &["tmf628"];

/// Configuration for a TMF module, keyed by its cargo feature name
/// (e.g. `"tmf628"`). Returns `None` for modules without a configuration,
/// which are still hand-maintained in tmflib's `src/` tree.
pub fn config_for_feature(feature: &str) -> Option<ModuleConfig> {
    match feature {
        "tmf628" => Some(tmf628_config()),
        _ => None,
    }
}

/// Configuration for the TMF628 Performance Management module.
pub fn tmf628_config() -> ModuleConfig {
    ModuleConfig::new("tmf628")
        .with_spec_file("TMF628_Performance-v5.0.0.oas.yaml")
        .with_tmf_module("performanceManagement")
        .with_managed(ManagedType::new("PerformanceMeasurement", "measurement").addressable())
}
