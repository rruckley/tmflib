//! Build module to generate new modules from OAS / Swagger files
//! 

use std::env;
use std::fs;
use std::path::Path;
use openapiv3::Schema;
use quote::quote;
use openapiv3::OpenAPI;
use serde_json;
use proc_macro2::{Ident,Span};
use convert_case::{Case,Casing};

fn generate_schema_mod(name : String, _schema : Option<Schema>) -> String {
    let ident = Ident::new(name.as_str(), Span::call_site());
    let schema_struct = quote! {
        pub struct #ident {

        }
    };
    let schema_impl = quote! {
        impl #ident {

        }
    };
    let mod_out = quote!{
        #schema_struct

        #schema_impl
    };
    mod_out.to_string()
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
        mod_list.push_str(format!("mod {};\n",snake_mod).as_str());
        let file_name = format!("{}.rs",snake_mod);
        let schema_path = Path::new(&out_dir).join(mod_dir).join(file_name.as_str());
        let camel_name = name.to_case(Case::Camel);
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