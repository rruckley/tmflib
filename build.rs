//! Build module to generate new modules from OAS / Swagger files
//! 

use std::collections::HashMap;
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

#[derive(Default,Debug,Clone)]
struct CommonEntry {
    name : String,
    use_path : String,
}

#[derive(PartialEq)]
enum BuildMode {
    HashOnly,
    FullBuild,
}

impl From<(&str,&str)> for CommonEntry {
    fn from(value: (&str,&str)) -> Self {
        let (name,use_path) = value;
        CommonEntry {
            name : name.to_string(),
            use_path : use_path.to_string(),
        }
    }
}

#[derive(Default)]
struct CommonClasses {
    classes : Vec<CommonEntry>,
}

impl From<Vec<(&str,&str)>> for CommonClasses {
    fn from(value: Vec<(&str,&str)>) -> Self {
        let mut out = CommonClasses::default();
        for value in value.iter() {
            let entry = CommonEntry::from(*value);
            out.classes.push(entry);
        }
        out
    }
}

#[derive(Default,Debug)]
struct StringWithUse {
    pub string : String,
    pub uses : Option<Vec<String>>,
    pub generic : Option<String>,
}

impl StringWithUse {
    pub fn with_ref(mut self, reference : impl Into<String>) -> StringWithUse {
        if self.uses.is_none() {
            self.uses = Some(vec![]);
        };
        self.uses.as_mut().unwrap().push(reference.into());
        self
    }
    pub fn with_generic(mut self, generic : impl Into<String>) -> StringWithUse {
        self.generic = Some(generic.into());
        self
    }
    // Merge one SWU into another
    pub fn merge(&mut self,swu : &mut StringWithUse) {
        // Merge new StringWithUse into this one
        if self.uses.is_none() {
            self.uses = Some(vec![]);
        };
        self.string.push_str(swu.string.as_str());
        // Simple copy through the genric value
        self.generic = swu.generic.clone();
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
            generic : None,
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

fn property_type_array(a: ArrayType, schema_hash: &mut Vec<String>) -> StringWithUse {
    let mut out = String::default();
    let mut out2 = StringWithUse::default();
    out.push_str("Vec<");
    match a.items {
        Some(i) => {
            let mut prop = property_ref(i,schema_hash);
            out2.merge(&mut prop);
            out.push_str(prop.string.as_str())
        },
        None => (),
    }
    out.push_str(">");
    out2.string = out;
    out2
}

fn property_type(t : Type, schema_hash : &mut Vec<String>) -> StringWithUse {
    // Return a Rust type
    match t {
        Type::String(_s) => format!("String").into(),
        Type::Boolean(_b)=> format!("bool").into(),
        Type::Integer(_i)=> format!("i32").into(),
        Type::Number(_f)=> format!("f32").into(),
        Type::Object(o) => schema_object_properties(o,schema_hash),
        Type::Array(a) => property_type_array(a,schema_hash),
    }
}

fn property_schema_allof(_all_of : Vec<ReferenceOr<Schema>>) -> StringWithUse {
    format!("\n// Property Schema AllOf not implemented\n").into()
}

fn property_schema_any(any : AnySchema,schema_vec: &mut Vec<String>) -> StringWithUse {
    let mut out = String::default();
    let mut out2 = StringWithUse::default();
    for (name, ref_or) in any.properties.into_iter() {
        let name = name
            .replace("@", "")
            .replace("type", "r#type")
            .to_case(Case::Snake);
        let mut prop_vec: Vec<String> = vec![];
        let mut props = property_ref(ref_or,&mut prop_vec);
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

fn property_schema(schema : &Schema, schema_vec: &mut Vec<String>) -> StringWithUse {
    match schema.clone().schema_kind {
        SchemaKind::Type(t) => property_type(t,schema_vec).into(),
        SchemaKind::AllOf { all_of } => property_schema_allof(all_of),
        SchemaKind::AnyOf { any_of } => property_schema_anyof(any_of),
        SchemaKind::OneOf { one_of } => property_schema_oneof(one_of),
        SchemaKind::Any(any) => property_schema_any(any,schema_vec),
        SchemaKind::Not { not } => property_schema_not(not),
    }
}

fn property_ref(s: ReferenceOr<Box<Schema>>,schema_vec: &mut Vec<String>) -> StringWithUse {
    match s {
        ReferenceOr::Item(i) => {
            property_schema(i.as_ref(),schema_vec)
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

fn reference_to_uses(reference : String) -> String {
    // Pull out common modules
    match reference.as_str() {
        "TimePeriod" => format!("use crate::TimePeriod;\n"),
        "RelatedParty"=> format!("use crate::common::related_party::RelatedParty;\n"),
        "Note" => format!("use crate::common::note::Note;\n"),
        _ => {
            // Default case, generate use path from Camel name.
            let snake = reference.to_case(Case::Snake);
            format!("use crate::tmf723::{}::{};\n",snake,reference)
        }
    }
}

fn schema_kind(name : String, kind : SchemaKind,schema_hash : &mut HashMap<String,Vec<String>>,mode : BuildMode) -> String {
    // let schema_name = name.clone();
    let out = match kind {
        SchemaKind::Type(t) => schema_type(name,t,schema_hash,mode),
        SchemaKind::AllOf { all_of } => schema_allof(name, all_of,schema_hash,mode),
        SchemaKind::AnyOf { any_of } => schema_anyof(name, any_of),
        SchemaKind::Not { not } => schema_not(name,not),
        SchemaKind::OneOf { one_of } => schema_oneof(name, one_of),
        SchemaKind::Any(any) => schema_any(name, any,schema_hash,mode),
    };
    //schema_hash.insert(schema_name,schema_vec);
    out
}

fn schema_type(name: String, t : Type, schema_hash: &mut HashMap<String,Vec<String>>, mode : BuildMode) -> String {
    let mut schema_vec : Vec<String>= vec![];
    let schema_name = name.clone();
    let mut out = match t {
        Type::Object(o) => schema_object(name,o,&mut schema_vec).into(),
        Type::Array(a) => schema_array(name, a,&mut schema_vec),
        Type::Boolean(_b) => format!("// Schema Type bool not implemented\n"),
        Type::Integer(_i) => format!("// Schema Type Integer not implemented\n"),
        Type::Number(_n) => format!("// Schema Type Number not implemented\n"),
        Type::String(_s) => format!("// Schema Type String not implemented\n"),
    };
    // Store properties for later reuse
    if mode == BuildMode::HashOnly {
        let vec_size = schema_vec.len();
        out.push_str(format!("\t// Inserting {} into hash, schema_vec is : {}",schema_name.clone(),vec_size).as_str());
        schema_hash.insert(schema_name, schema_vec);
        let hash_size = schema_hash.len();
        out.push_str(format!("Hash is now: {}",hash_size).as_str());
    }
    out
}

fn schema_object(name: String, object : ObjectType, schema_vec: &mut Vec<String>) -> String {
    let mut out = StringWithUse::default();
    let props = schema_object_properties(object,schema_vec);
    out.string.push_str(mod_uses(props.uses).as_str());
    out.string.push_str("\n// schema_object()\n");
    out.string.push_str("#[derive(Debug,Default,Deserialize,Serialize)]\n");
    out.string.push_str(format!("pub struct {} {{\n",name).as_str());
    out.string.push_str(props.string.as_str());
    out.string.push_str("\n}\n");
    out.into()
}

fn schema_object_properties(object: ObjectType,schema_vec: &mut Vec<String>) -> StringWithUse {
    let mut out = String::default();
    let mut out2 = StringWithUse::default();
    for (name, schema) in object.properties.into_iter() {
        let name = name
            .replace("@", "")
            .replace("type", "r#type")
            .to_case(Case::Snake);
        let mut prop_ref = property_ref(schema,schema_vec);
        let line = format!("\n\t{}: {},",name,prop_ref.string);
        out.push_str(line.as_str());
        
        let vec_size = schema_vec.len();
        schema_vec.push(line);
        // out.push_str(format!("\t// Added line to vec, now: {}\n",vec_size).as_str());
        out2.merge(&mut prop_ref)
    }
    out2.string = out;
    out2
}


fn schema_array(name: String, array : ArrayType,schema_vec: &mut Vec<String>) -> String {
    let mut out = String::default();
    out.push_str(property_ref(array.items.unwrap(),schema_vec).string.as_str());
    format!("\t{}:\nVec<{}>\n",name,out)
}

fn schema_allof(name : String, all_of : Vec<ReferenceOr<Schema>>,schema_hash : &mut HashMap<String,Vec<String>>,mode: BuildMode) -> String {
    let mut out = String::default();
    let mut uses = StringWithUse::default();
    out.push_str("// Schema AllOf\n");
    let mut prop_vec : Vec<String> = vec![];
    for r in all_of {
        out.push_str(match r {
            ReferenceOr::Item(i) => {
               
               let mut props = property_schema(&i,&mut prop_vec);
               let vec_len = prop_vec.len();
               props.string.push_str(format!("\t// Found {} properties, adding to hash",vec_len).as_str());
               
               uses.merge(&mut props);
               props.string
            },
            // This reference needs to pull the schema properties in, not just link
            // Need to have a hash of all schemas to pull in
            ReferenceOr::Reference { reference } => {
                let (_,split_ref) = reference.rsplit_once("/").unwrap();
                // Need to pull in referenced schema
                //uses.push_str(reference_to_uses(split_ref.to_string()).as_str());
                // Use split_ref to lookup hash to find items
                match schema_hash.get_key_value(split_ref) {
                    Some((_name,value)) => {
                        // Entry found, add into output
                        let mut out = String::default();
                        out.push_str(format!("\n\t//{}:\t{}","START",split_ref).as_str());
                        // We want to include the referenced fields in our vec
                        if value.is_empty() {
                            out.push_str("\t// Empty SchemaVec\n");
                        }
                        for val in value {
                            // out.push_str("// Entry");
                            out.push_str(val)
                        }
                        out.push_str(format!("\n\t//{}:\t{},\n","FINISH",split_ref).as_str());
                        out
                    },
                    None => {
                        format!("\t//{}:\t{},\n",name,split_ref)
                    }
                }
                
            }
        }.as_str());
    }
    schema_hash.insert(name.clone(), prop_vec);
    out.push_str("\n}\n");
    let struct_def = match uses.generic {
        Some(g) => format!("\n#[derive(Debug,Default,Deserialize,Serialize)]\npub struct {}<{}> {{\n",name,g),
        None => format!("\n#[derive(Debug,Default,Deserialize,Serialize)]\npub struct {} {{\n",name),
    }; 
    // Generate final output, uses, then struct
    format!("\n// Uses\n{}\n{}\n{}",mod_uses(uses.uses),struct_def,out)
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

fn schema_any(name : String, any : AnySchema, schema_hash : &mut HashMap<String,Vec<String>>, mode: BuildMode) -> String {
    let mut out = String::default();
    let mut uses = StringWithUse::default();
    let mut outer_vec : Vec<String> = vec![];
    out.push_str("// Schema Any\n");
    out.push_str("#[derive(Debug,Default,Deserialize,Serialize)]\n");
    out.push_str(format!("pub struct {} {{\n",name).as_str());
    for (name,r) in any.properties {
        
        out.push_str(match r {
            // We can add the item onto the has here
            ReferenceOr::Item(i) => {
                let mut schema_vec : Vec<String> = vec![];
               let mut props = property_schema(&i,&mut schema_vec);
               uses.merge(&mut props);
               let name = name
                    .replace("@", "")
                    .replace("type", "r#type")
                    .to_case(Case::Snake);
                if mode == BuildMode::HashOnly {
                    schema_hash.insert(name.clone(),schema_vec);
                }
                format!("\t{}:\tOption<{}>,\n",name,props.string)
            },
            // This reference needs to pull the schema properties in, not just link
            // Need to have a hash of all schemas to pull in
            ReferenceOr::Reference { reference } => {
                let (_,split_ref) = reference.rsplit_once("/").unwrap();
                // Need to pull in referenced schema
                //uses.push_str(reference_to_uses(split_ref.to_string()).as_str());
                let mut out = format!("\t//START:\t{},\n",split_ref);
                let vec = schema_hash.get(split_ref);
                match vec {
                    Some(v) => {
                        for val in v {
                            // out.push_str("// Entry");
                            out.push_str(val.as_str());
                        }
                    },
                    None => {
                        out.push_str("\n// Empty vec for schema_any\n");
                    }
                }
                out.push_str(format!("\t//FINISH:\t{},\n",split_ref).as_str());
                
                out
            }
        }.as_str());
    }
    for ref_or in any.one_of {
        // We can add the item onto the has here
     out.push_str(match ref_or {
            ReferenceOr::Item(i) => {
                let mut props = property_schema(&i,&mut outer_vec);
                uses.merge(&mut props);
                let name = name
                    .replace("@", "")
                    .replace("type", "r#type")
                    .to_case(Case::Snake);
                //schema_hash.insert(name.clone(),schema_vec);
                format!("\t{}:\tOption<{}>,\n",name,props.string)
            },
            // This reference needs to pull the schema properties in, not just link
            // Need to have a hash of all schemas to pull in
            ReferenceOr::Reference { reference } => {
                let (_,split_ref) = reference.rsplit_once("/").unwrap();
                // Need to pull in referenced schema
                //uses.push_str(reference_to_uses(split_ref.to_string()).as_str());
                let mut out = format!("\t//Pull in this schema:\t{}",split_ref);
                let vec = schema_hash.get(split_ref);
                match vec {
                    Some(v) => {
                        outer_vec.append(v.clone().as_mut());
                        for val in v {
                            // out.push_str("// Entry");
                            out.push_str(val.as_str());
                        }
                    },
                    None => {
                        out.push_str("\n\t// Vec was empty");
                    }
                }
                
                out
            }  
        }.as_str());
    }
    for ref_or in any.all_of {
        // We can add the item onto the has here
     out.push_str(match ref_or {
            ReferenceOr::Item(i) => {
                let mut props = property_schema(&i,&mut outer_vec);
                uses.merge(&mut props);
                let name = name
                    .replace("@", "")
                    .replace("type", "r#type")
                    .to_case(Case::Snake);
                //schema_hash.insert(name.clone(),schema_vec);
                format!("\t{}:\tOption<{}>,\n",name,props.string)
            },
            // This reference needs to pull the schema properties in, not just link
            // Need to have a hash of all schemas to pull in
            ReferenceOr::Reference { reference } => {
                let (_,split_ref) = reference.rsplit_once("/").unwrap();
                // Need to pull in referenced schema
                //uses.push_str(reference_to_uses(split_ref.to_string()).as_str());
                let mut out = format!("\t//Pull in this schema:\t{}",split_ref);
                let vec = schema_hash.get(split_ref);
                match vec {
                    Some(v) => {
                        outer_vec.append(v.clone().as_mut());
                        for val in v {
                            // out.push_str("// Entry");
                            out.push_str(val.as_str());
                        }
                    },
                    None => {
                        out.push_str("\n\t// Vec was empty");
                    }
                }
                
                out
            }  
        }.as_str());
    }
    schema_hash.insert(name,outer_vec);
    out.push_str("\n}\n");
    format!("\n// Uses\n{}\n{}",mod_uses(uses.uses),out)
}

fn schema_not(name : String, _not: Box<ReferenceOr<Schema>>) -> String {
    format!("// Kind Not not implemented for {}\n",name) 
}

fn schema_oneof(name : String, oneof : Vec<ReferenceOr<Schema>>) -> String {
    let mut out = String::default();
    out.push_str("// Generated Enum\n");
    for e in oneof.into_iter() {
        match e {
            ReferenceOr::Item(_i) => {
                out.push_str("// OneOf Reference")

            },
            ReferenceOr::Reference { reference } => {
                // Not sure how to manage a reference inside an enum
            }       
        }
    }
    out.push_str(format!("pub enum {} {{\n }}\n",name).as_str());
    format!("// Kind OneOf not implemented for {}\n",name)
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

fn generate_schema_mod(name : String, schema : Option<Schema>,schema_hash : &mut HashMap<String,Vec<String>>,mode : BuildMode) -> String {
    // Take schema name and schema and generate the Rust code
    let mut out = String::default();

    // First, determine if we have a struct or an enum
    match schema {
        Some(s) => {
            // We have a schema, we can convert to string
            match mode {
                BuildMode::FullBuild => {
                    out.push_str(format!("//! Generated Module: {}\n",name).as_str());
                    out.push_str(schema_description(&s).as_str());
                    out.push_str(schema_kind(name.clone(),s.schema_kind.clone(),schema_hash,mode).as_str());
                },
                BuildMode::HashOnly => {
                    let _ = schema_kind(name.clone(),s.schema_kind.clone(),schema_hash,mode);
                }
            }

            
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
    let mut schema_hash : HashMap<String,Vec<String>> = HashMap::new();
    // Will throw error if path already exists but we don't care about that situation
    let _ = fs::create_dir(mod_path);
    for (name,schema) in schemas.iter() {
        // Initial pass just builds out the Hash
        let camel_name = name.to_case(Case::UpperCamel);
        let _ = generate_schema_mod(camel_name,schema.clone().into_item(),&mut schema_hash,BuildMode::HashOnly);    
    }
    for (name,schema) in schemas.iter() {
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
        let out = generate_schema_mod(camel_name,schema.clone().into_item(),&mut schema_hash,BuildMode::FullBuild);
        fs::write(&schema_path,out).unwrap();
    }
    // Now we have a list of modules to include, we can create tmf723/mod.rs
    let mod_rs_path = Path::new(&out_dir).join(mod_dir).join("mod.rs");
    let mod_rs_contents = format!("pub struct GenericType<T : Sized> {{ }}\n\n{}",mod_list).to_string();
    let _ = fs::write(&mod_rs_path,mod_rs_contents);
    let auto_lib_contents = quote!{
        pub mod tmf723;

    }.to_string();
    let _ = fs::write(&dest_path,auto_lib_contents);
    // Generate output for each Swagger / OAS file found
    
    println!("cargo:rerun-if-changed=build.rs");
}