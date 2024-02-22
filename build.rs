//! Build module to generate new modules from OAS / Swagger files
//! 

use std::env;
use std::fs;
use std::path::Path;
use openapiv3::ObjectType;
use openapiv3::ReferenceOr;
use openapiv3::Schema;
use openapiv3::SchemaKind;
use openapiv3::Type;
// use openapiv3::SchemaKind;
use quote::quote;
use openapiv3::OpenAPI;
use serde_json;
//use proc_macro2::{Ident,Span};
use convert_case::{Case,Casing};

fn schema_description(schema : &Schema) -> String {
    let d = schema.schema_data.description.as_ref();
    match d {
        Some(d) => format!("\n/*\n\t{}\n*/\n",d),
        None => String::default(),
    }    
}

fn property_type(s: ReferenceOr<Box<Schema>>) -> String {
    match s {
        ReferenceOr::Item(i) => {
            String::default()
        },
        ReferenceOr::Reference { reference } => {
            String::default()
        }
    }
}

fn schema_object_properties(object: ObjectType) -> String {
    let mut out = String::default();
    for (name,schema) in object.properties.into_iter() {
        let name = name
            .replace("@", "")
            .replace("type", "r#type")
            .to_case(Case::Snake);
        out.push_str(format!("\t{}: String,\n",name).as_str());
    }
    out
}

fn schema_object(name: String, object : ObjectType) -> String {
    let mut out = String::default();
    out.push_str(mod_uses().as_str());
    out.push_str("#[derive(Debug,Default,Deserialize,Serialize)]\n");
    out.push_str(format!("pub struct {} {{\n",name).as_str());
    out.push_str(schema_object_properties(object).as_str());
    out.push_str("}\n");
    out
}

fn schema_type(name: String, t : Type) -> String {
    match t {
        Type::Object(o) => schema_object(name,o),
        _ => format!("// Type {} not implemented",name),
    }
}

fn schema_kind(name : String, kind : SchemaKind) -> String {
    match kind {
        SchemaKind::Type(t) => schema_type(name,t),
        _ => format!("// Kind {} not implemented\n",name)
    }
}

fn mod_uses() -> String {
    format!("use serde::{{Deserialize,Serialize}};\n\n")
}

fn generate_schema_mod(name : String, schema : Option<Schema>) -> String {
    // Take schema name and schema and generate the Rust code
    let mut out = String::default();

    // First, determine if we have a struct or an enum
    match schema {
        Some(s) => {
            // We have a schema, we can convert to string
            out.push_str(format!("//! Generated Module: {}\n",name).as_str());
            
            out.push_str(schema_description(&s).as_str());
            out.push_str(schema_kind(name.clone(),s.schema_kind.clone()).as_str());
        },
        None => {
            out.push_str("//! Empty Module\n")
        },
    };

    out
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("auto-lib.rs");

    // Open a OAS file
    let data = include_str!("open_api/TMF723-Policy_Management-v5.0.0.oas.json");
    let openapi : OpenAPI = serde_json::from_str(data).expect("Could not parse YAML");
    let components = openapi.components.expect("No components found!");
    let schemas = components.schemas;
    // Iterate through all schemas generating a file in module directory
    let mod_dir : &str = "tmf723";
    let mod_path = Path::new(&out_dir).join(mod_dir);
    let mut mod_list = String::default();
    // Will throw error if path already exists but we don't care about that situation
    let _ = fs::create_dir(mod_path);
    for (name,schema) in schemas {
        // The output here is the contents for the schema file 'name'
        // We need to write this to the appropriate filename'
        let snake_mod = name.to_case(Case::Snake);
        mod_list.push_str(format!("pub mod {};\n",snake_mod).as_str());
        let file_name = format!("{}.rs",snake_mod);
        let schema_path = Path::new(&out_dir).join(mod_dir).join(file_name.as_str());
        let camel_name = name.to_case(Case::UpperCamel);
        let out = generate_schema_mod(camel_name,schema.into_item());
        fs::write(&schema_path,out).unwrap();
    }
    // Now we have a list of modules to include, we can create tmf723/mod.rs
    let mod_rs_path = Path::new(&out_dir).join(mod_dir).join("mod.rs");
    let mod_rs_contents = format!("{}",mod_list).to_string();
    let _ = fs::write(&mod_rs_path,mod_rs_contents);
    let auto_lib_contents = quote!{
        mod tmf723;

    }.to_string();
    let _ = fs::write(&dest_path,auto_lib_contents);
    // Generate output for each Swagger / OAS file found
    
    println!("cargo:rerun-if-changed=build.rs");
}