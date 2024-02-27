//! Build module to generate new modules from OAS / Swagger files
//! 

use std::env;
use std::fs;
use std::path::Path;
use openapiv3::AnySchema;
use openapiv3::ArrayType;
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

#[derive(Default,Debug)]
struct StringWithUse {
    pub string : String,
    pub uses : Option<Vec<String>>,
}

impl StringWithUse {
    pub fn with_ref(mut self, reference : String) -> StringWithUse {
        if self.uses.is_none() {
            self.uses = Some(vec![]);
        };
        self.uses.as_mut().unwrap().push(reference);
        self
    }
    // Merge one SWU into another
    pub fn merge(&mut self,swu : &mut StringWithUse) {
        // Merge new StringWithUse into this one
        if self.uses.is_none() {
            self.uses = Some(vec![]);
        };
        self.string.push_str(swu.string.as_str());
        if swu.uses.is_some() {
            let new_uses = swu.uses.as_mut().unwrap();
            self.uses.as_mut().unwrap().append(new_uses);    
        }
    }
}

impl From<String> for StringWithUse {
    fn from(value: String) -> Self {
        StringWithUse {
            string : value,
            uses : None,
        }
    }
}

impl Into<String> for StringWithUse {
    fn into(self) -> String {
        self.string
    }
}

fn schema_description(schema : &Schema) -> String {
    let d = schema.schema_data.description.as_ref();
    match d {
        Some(d) => format!("\n/*\n\t{}\n*/\n",d),
        None => String::default(),
    }    
}

fn property_type_array(a: ArrayType) -> StringWithUse {
    let mut out = String::default();
    let mut out2 = StringWithUse::default();
    out.push_str("Vec<");
    match a.items {
        Some(i) => {
            let mut prop = property_ref(i);
            out2.merge(&mut prop);
            out.push_str(prop.string.as_str())
        },
        None => (),
    }
    out.push_str(">");
    out2.string = out;
    out2
}

fn property_type(t : Type) -> StringWithUse {
    // Return a Rust type
    match t {
        Type::String(_s) => format!("String").into(),
        Type::Boolean(_b)=> format!("bool").into(),
        Type::Integer(_i)=> format!("i32").into(),
        Type::Number(_f)=> format!("f32").into(),
        Type::Object(o) => schema_object_properties(o),
        Type::Array(a) => property_type_array(a),
    }
}

fn property_schema_allof(_all_of : Vec<ReferenceOr<Schema>>) -> StringWithUse {
    format!("\n// Property Schema AllOf not implemented\n").into()
}

fn property_schema_any(any : AnySchema) -> StringWithUse {
    let mut out = String::default();
    let mut out2 = StringWithUse::default();
    for (name, ref_or) in any.properties.into_iter() {
        let name = name
            .replace("@", "")
            .replace("type", "r#type")
            .to_case(Case::Snake);
        let mut props = property_ref(ref_or);
        out.push_str(format!("\t{}: {},\n",name,props.string).as_str());
        out2.merge(&mut props);
    }
    out2.string = out;
    out2
}

fn property_schema_not( _not: Box<ReferenceOr<Schema>>) -> StringWithUse {
    format!("\n// Property Schema Not not implemented\n").into()
}

fn property_schema_anyof(_any_of : Vec<ReferenceOr<Schema>>) -> StringWithUse {
    format!("\n// Property Schema AnyOf not implemented\n").into()
}

fn property_schema_oneof(_one_of: Vec<ReferenceOr<Schema>>) -> StringWithUse {
    format!("\n// Property Schema OneOf not implemented\n").into()
}

fn property_schema(schema : &Schema) -> StringWithUse {
    match schema.clone().schema_kind {
        SchemaKind::Type(t) => property_type(t).into(),
        SchemaKind::AllOf { all_of } => property_schema_allof(all_of),
        SchemaKind::AnyOf { any_of } => property_schema_anyof(any_of),
        SchemaKind::OneOf { one_of } => property_schema_oneof(one_of),
        SchemaKind::Any(any) => property_schema_any(any),
        SchemaKind::Not { not } => property_schema_not(not),
    }
}

fn property_ref(s: ReferenceOr<Box<Schema>>) -> StringWithUse {
    match s {
        ReferenceOr::Item(i) => {
            property_schema(i.as_ref())
        },
        ReferenceOr::Reference { reference } => {
            let (_,split_ref) = reference.rsplit_once("/").unwrap();
            // Since we are now referencing a remote class, we need to include it via a 'use' statement.
            // Need to return the use class
            StringWithUse::from(format!("{}",split_ref))
                .with_ref(split_ref.to_string())
        }
    }
}

fn schema_object_properties(object: ObjectType) -> StringWithUse {
    if object.properties.is_empty() {
        return StringWithUse::default()
    }
    let mut out = String::default();
    let mut out2 = StringWithUse::default();
    for (name, schema) in object.properties.into_iter() {
        let name = name
            .replace("@", "")
            .replace("type", "r#type")
            .to_case(Case::Snake);
        let mut prop_ref = property_ref(schema);
        out.push_str(format!("\n\t{}: {},",name,prop_ref.string).as_str());
        out2.merge(&mut prop_ref)
    }
    out2.string = out;
    out2
}

fn schema_object(name: String, object : ObjectType) -> String {
    let mut out = StringWithUse::default();
    let props = schema_object_properties(object);
    out.string.push_str(mod_uses(props.uses).as_str());
    out.string.push_str("\n// schema_object()\n");
    out.string.push_str("#[derive(Debug,Default,Deserialize,Serialize)]\n");
    out.string.push_str(format!("pub struct {} {{\n",name).as_str());
    out.string.push_str(props.string.as_str());
    out.string.push_str("\n}\n");
    out.into()
}

fn schema_type(name: String, t : Type) -> String {
    match t {
        Type::Object(o) => schema_object(name,o).into(),
        Type::Array(_a) => format!("// Schema Type Array not implemented\n"),
        Type::Boolean(_b) => format!("// Schema Type bool not implemented\n"),
        Type::Integer(_i) => format!("// Schema Type Integer not implemented\n"),
        Type::Number(_n) => format!("// Schema Type Number not implemented\n"),
        Type::String(_s) => format!("// Schema Type String not implemented\n"),
    }
}

fn reference_to_uses(reference : String) -> String {
    let snake = reference.to_case(Case::Snake);
    format!("use crate::tmf723::{}::{};\n",snake,reference)
}

fn schema_allof(name : String, all_of : Vec<ReferenceOr<Schema>>) -> String {
    let mut out = String::default();
    let mut uses = StringWithUse::default();
    out.push_str("// Schema AllOf\n");
    out.push_str("#[derive(Debug,Default,Deserialize,Serialize)]\n");
    out.push_str(format!("pub struct {} {{\n",name).as_str());

    for r in all_of {
        out.push_str(match r {
            ReferenceOr::Item(i) => {
               let mut props = property_schema(&i);
               uses.merge(&mut props);
               props.string
            },
            // This reference needs to pull the schema properties in, not just link
            // Need to have a hash of all schemas to pull in
            ReferenceOr::Reference { reference } => {
                let (_,split_ref) = reference.rsplit_once("/").unwrap();
                // Need to pull in referenced schema
                //uses.push_str(reference_to_uses(split_ref.to_string()).as_str());
                format!("\t//{}: {},\n",name,split_ref).into()
            }
        }.as_str());
    }
    out.push_str("\n}\n");
    // Generate final output, uses, then struct
    format!("\n// Uses\n{}\n{}",mod_uses(uses.uses),out)
}

fn schema_anyof(name: String, _any_of : Vec<ReferenceOr<Schema>>) -> String {
    let mut out = String::default();
    out.push_str("use serde::{Deserialize,Serialize};\n\n");
    out.push_str("// Schema AnyOf\n");
    out.push_str("#[derive(Debug,Default,Deserialize,Serialize)]\n");
    out.push_str(format!("pub struct {} {{\n",name).as_str());
    out.push_str("\n}\n");
    out    
}

fn schema_any(name : String, any : AnySchema) -> String {
    let mut out = String::default();
    let mut uses = StringWithUse::default();
    out.push_str("// Schema Any\n");
    out.push_str("#[derive(Debug,Default,Deserialize,Serialize)]\n");
    out.push_str(format!("pub struct {} {{\n",name).as_str());
    for (name,r) in any.properties {
        out.push_str(match r {
            ReferenceOr::Item(i) => {
               let mut props = property_schema(&i);
               uses.merge(&mut props);
               let name = name
                    .replace("@", "")
                    .replace("type", "r#type")
                    .to_case(Case::Snake);
               format!("\t{}:\tOption<{}>,\n",name,props.string)
            },
            // This reference needs to pull the schema properties in, not just link
            // Need to have a hash of all schemas to pull in
            ReferenceOr::Reference { reference } => {
                let (_,split_ref) = reference.rsplit_once("/").unwrap();
                // Need to pull in referenced schema
                //uses.push_str(reference_to_uses(split_ref.to_string()).as_str());
                format!("\t//{}:\t{},\n",name,split_ref).into()
            }
        }.as_str());
    }
    out.push_str("}\n");
    format!("\n// Uses\n{}\n{}",mod_uses(uses.uses),out)
}

fn schema_not(name : String, _not: Box<ReferenceOr<Schema>>) -> String {
    format!("// Kind Not not implemented for {}\n",name) 
}

fn schema_oneof(name : String, _oneof : Vec<ReferenceOr<Schema>>) -> String {
    format!("// Kind OneOf not implemented for {}\n",name)
}

fn schema_kind(name : String, kind : SchemaKind) -> String {
    match kind {
        SchemaKind::Type(t) => schema_type(name,t),
        SchemaKind::AllOf { all_of } => schema_allof(name, all_of),
        SchemaKind::AnyOf { any_of } => schema_anyof(name, any_of),
        SchemaKind::Not { not } => schema_not(name,not),
        SchemaKind::OneOf { one_of } => schema_oneof(name, one_of),
        SchemaKind::Any(any) => schema_any(name, any),
    }
}

fn mod_uses(uses : Option<Vec<String>>) -> String {
    let mut out = String::default();
    out.push_str(format!("use serde::{{Deserialize,Serialize}};\n\n").as_str());
    match uses {
        Some(mut u) => {
            // Need to ensure we don't repeat ourselves
            u.sort();
            u.dedup();
            for uses in u.into_iter() {
                out.push_str(&reference_to_uses(uses).as_str());
            }    
        },
        None => (),
    };
    out
}

fn generate_schema_mod(name : String, schema : Option<Schema>) -> String {
    // Take schema name and schema and generate the Rust code
    let mut out = String::default();

    // First, determine if we have a struct or an enum
    match schema {
        Some(s) => {
            // We have a schema, we can convert to string
            out.push_str(format!("//! Generated Module: {}\n",name).as_str());
            // Insert uses here.
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
        if name.find("FVO").is_some() || name.find("MVO").is_some() {
            continue;
        }
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