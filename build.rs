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

fn property_type_array(a: ArrayType) -> String {
    let mut out = String::default();
    out.push_str("Vec<");
    match a.items {
        Some(i) => {
            out.push_str(property_ref(i).string.as_str())
        },
        None => (),
    }
    out.push_str(">");
    out
}

fn property_type(t : Type) -> String {
    // Return a Rust type
    match t {
        Type::String(s) => format!("String"),
        Type::Boolean(b)=> format!("bool"),
        Type::Integer(i)=> format!("i32"),
        Type::Number(f)=> format!("f32"),
        Type::Object(o) => schema_object_properties(o).into(),
        Type::Array(a) => property_type_array(a),
        _ => format!("!"),
    }
}

fn property_schema_any(any : AnySchema) -> String {
    let mut out = String::default();
    for (name, ref_or) in any.properties.into_iter() {
        let name = name
            .replace("@", "")
            .replace("type", "r#type")
            .to_case(Case::Snake);
        out.push_str(format!("\n\t{}: {},",name,property_ref(ref_or).string).as_str());
    }
    out
}

fn property_schema(schema : &Schema) -> String {
    match schema.clone().schema_kind {
        SchemaKind::Type(t) => property_type(t),
        SchemaKind::AllOf { all_of } => format!("\n// Property Schema AllOf not implemented\n"),
        SchemaKind::AnyOf { any_of } => format!("\n// Property Schema AnyOf not implemented\n"),
        SchemaKind::OneOf { one_of } => format!("\n// Property Schema OneOf not implemented\n"),
        SchemaKind::Any(any) => property_schema_any(any),
        SchemaKind::Not { not } => format!("\n// Property Schema Not not implemented\n"),
    }
}

fn property_ref(s: ReferenceOr<Box<Schema>>) -> StringWithUse {
    match s {
        ReferenceOr::Item(i) => {
            property_schema(i.as_ref()).into()
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
        return format!("String, // Should be an object {{ }}").into();
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
    out.string.push_str("#[derive(Debug,Default,Deserialize,Serialize)]\n");
    out.string.push_str(format!("pub struct {} {{\n",name).as_str());
    out.string.push_str(props.string.as_str());
    out.string.push_str("\n}\n");
    out.into()
}

fn schema_type(name: String, t : Type) -> String {
    match t {
        Type::Object(o) => schema_object(name,o).into(),
        Type::Array(a) => format!("// Schema Type Array not implemented\n"),
        Type::Boolean(b) => format!("// Schema Type bool not implemented\n"),
        Type::Integer(i) => format!("// Schema Type Integer not implemented\n"),
        Type::Number(n) => format!("// Schema Type Number not implemented\n"),
        Type::String(s) => format!("// Schema Type String not implemented\n"),
    }
}

fn reference_to_uses(reference : String) -> String {
    let snake = reference.to_case(Case::Snake);
    format!("use crate::tmf723::{}::{};\n",snake,reference)
}

fn schema_allof(name : String, all_of : Vec<ReferenceOr<Schema>>) -> String {
    let mut out = String::default();
    let mut uses = String::default();
    out.push_str("// Schema AllOf\n");
    out.push_str(format!("pub struct {} {{\n",name).as_str());
    for r in all_of {
        out.push_str(match r {
            ReferenceOr::Item(i) => {
               property_schema(&i) 
            },
            ReferenceOr::Reference { reference } => {
                let (_,split_ref) = reference.rsplit_once("/").unwrap();
                // Need to pull in referenced object
                // This reference needs to go into a 'use' statement
                uses.push_str(reference_to_uses(split_ref.to_string()).as_str());
                format!("\n\t//{}: {},\n",name,split_ref)
            }
        }.as_str());
    }
    out.push_str("\n}\n");
    // Generate final output, uses, then struct
    format!("\n// Uses\n{}\n{}",uses,out)
}

fn schema_anyof(name: String, any_of : Vec<ReferenceOr<Schema>>) -> String {
    let mut out = String::default();
    out.push_str("// Schema AnyOf\n");
    out.push_str(format!("pub struct {} {{ }}\n",name).as_str());
    out    
}

fn schema_any(name : String, any : AnySchema) -> String {
    let mut out = String::default();
    out.push_str("// Schema Any\n");
    out.push_str(format!("pub struct {} {{\n",name).as_str());
    out.push_str("}\n");
    out
}

fn schema_kind(name : String, kind : SchemaKind) -> String {
    match kind {
        SchemaKind::Type(t) => schema_type(name,t),
        SchemaKind::AllOf { all_of } => schema_allof(name, all_of),
        SchemaKind::AnyOf { any_of } => schema_anyof(name, any_of),
        SchemaKind::Not { not } => format!("// Kind Not not implemented\n"),
        SchemaKind::OneOf { one_of } => format!("// Kind OneOf not implemented\n"),
        SchemaKind::Any(any) => schema_any(name, any),
    }
}

fn mod_uses(uses : Option<Vec<String>>) -> String {
    let mut out = String::default();
    out.push_str(format!("use serde::{{Deserialize,Serialize}};\n\n").as_str());
    match uses {
        Some(u) => {
            for uses in u.into_iter() {
                let snake = uses.to_case(Case::Snake);
                out.push_str(format!("use crate::tmf723::{}::{};\n",snake,uses).as_str());
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