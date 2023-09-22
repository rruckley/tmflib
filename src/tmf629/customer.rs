//! Customer Module
//! 
use uuid::Uuid;
use serde::{Deserialize,Serialize};
use sha256::digest;

use super::MOD_PATH;
use super::characteristic::Characteristic;
use super::contact::ContactMedium;

const CUST_PATH : &str = "customer";

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct Customer {
    href    : String,
    id      : String,
    name    : String,
    pub status : Option<String>,
    pub status_reason : Option<String>,
    pub valid_for   : Option<String>,
    contact_medium  : Vec<ContactMedium>,
    characteristic  : Vec<Characteristic>,
}

impl Customer {
    pub fn new(name : String) -> Customer {
        // 91 143 471 845
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{id}",MOD_PATH,CUST_PATH);
        let hash_input = format!("{}:{}",id,name);
        let sha = digest(hash_input);
        let sha_slice = sha.as_str()[..4].to_string().to_ascii_uppercase();
        let code = Characteristic {
            name    : String::from("code"),
            value_type : String::from("string"),
            value : sha_slice,
        };
        let hash = Characteristic {
            name    : String::from("hash"),
            value_type : String::from("string"),
            value   : sha,
        };
        Customer {
            id,
            href,
            name,
            status : None,
            status_reason: None,
            valid_for: None,
            contact_medium : vec![],
            characteristic : vec![code,hash],
        }
    }
}