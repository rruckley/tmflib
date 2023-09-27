//! Customer Module
//! 
use uuid::Uuid;
use serde::{Deserialize,Serialize};
use sha256::digest;

use super::LIB_PATH;
use super::MOD_PATH;
use super::characteristic::Characteristic;
use super::contact::ContactMedium;

const CUST_PATH : &str = "customer";
pub const CUST_STATUS : &str = "New";

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct Customer {
    href    : Option<String>,
    id      : Option<String>,
    name    : String,
    pub status : Option<String>,
    pub status_reason : Option<String>,
    pub valid_for   : Option<String>,
    contact_medium  : Option<Vec<ContactMedium>>,
    characteristic  : Option<Vec<Characteristic>>,
}

impl Customer {
    pub fn new(name : String) -> Customer {
        // 91 143 471 845
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,CUST_PATH,id);
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
            id : Some(id),
            href : Some(href),
            name,
            status : Some(CUST_STATUS.to_string()),
            status_reason: None,
            valid_for: None,
            contact_medium : Some(vec![]),
            characteristic : Some(vec![code,hash]),
        }
    }

    pub fn generate_id(&mut self) {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,CUST_PATH,id);
        self.id = Some(id);
        self.href = Some(href);
    }

    pub fn generate_code(&mut self) {
        // Generate a new code based on name

        // Generate Id if none exists
        if self.id.is_none() {
            self.generate_id();
        };
        let hash_input = format!("{}:{}",self.id.as_ref().unwrap(),self.name);
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
        // Create vec if it doesn't exist
        if self.characteristic.is_none() {
            self.characteristic = Some(vec![]);
        }
    
        self.characteristic.as_mut().unwrap().push(code);
        self.characteristic.as_mut().unwrap().push(hash);
    }
}