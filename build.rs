//! Build module to generate new modules from OAS / Swagger files
//! 

use std::borrow::Borrow;
use std::env;
use std::fs;
use std::path::Path;
use openapiv3::Schema;
use openapiv3::Type;
use openapiv3::SchemaKind;
use quote::quote;
use openapiv3::OpenAPI;
use serde_json;
//use proc_macro2::{Ident,Span};
use convert_case::{Case,Casing};

fn type_to_string3(name: String, t : Type) -> String {
    match t {
        Type::String(s) => format!("String"),
        _ => format!("!"),
    }
}

fn type_to_string2(name: String, t : &Schema) -> String {
    match t.schema_kind.clone() {
        SchemaKind::Type(t) => type_to_string3(name, t),
        _ => format!("\t{}: !",name),  
    }
}
fn type_to_string1(name: String, t : Type) -> String {
    let mut output = String::default();
    match t {
        Type::Object(o) => {
            // Generate a pub struct
            output.push_str("#[derive(Debug,Default)]\n");
            output.push_str(format!("pub struct {} {{\n",name).as_str());
            for (name,item) in o.properties.into_iter() {
                // Generate properties
                // Remove @, its problematif for Rust
                let name = name.replace("@", "");
                let name = name.replace("type","r#type");
                let name = name.to_case(Case::Snake);
                match item.into_item() {
                    Some(i) => {
                        let schema = i.as_ref();
                        output.push_str(format!("\t{}: {},\n",name,type_to_string2(name.clone(),schema)).as_str());
                    },
                    None => (),
                }
                
            }
            output.push_str("}");
        },
        _ => {
            
        }
    }
    output
}

fn schema_to_string(name : String, schema : Schema) -> String {
    let mut out = String::default();
    let schema = match schema.schema_kind {
       SchemaKind::AllOf { all_of } => {
        // This matches a structure
        let mut type_list = String::default();
        all_of.into_iter().for_each(|f| {
            match f.into_item() {
                Some(i) => {
                    match i.schema_kind {
                        SchemaKind::Type(t) => {
                            type_list.push_str(format!("{}\n",type_to_string1(name.clone(),t)).as_str())
                        },
                        _ => {
                            // Not supported
                        }
                    }
                },
                None => {},
            }
        });
        type_list
       },
       SchemaKind::OneOf { one_of } => {
        // This matches an enum
        format!("
    #[derive(Debug,Clone)]
    pub enum {} {{

    }}
        ",name)
       }
       _ => {
        String::default()
       }
    };
    out.push_str(schema.as_str());
    out
}

fn generate_schema_mod(name : String, schema : Option<Schema>) -> String {
    // Take schema name and schema and generate the Rust code
    let mut out = String::default();

    // First, determine if we have a struct or an enum
    match schema {
        Some(s) => {
            // We have a schema, we can convert to string
            out.push_str(schema_to_string(name,s).as_str())
        },
        None => {},
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