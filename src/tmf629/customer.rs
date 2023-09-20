//! Customer Module
//! 
use uuid::Uuid;
use serde::{Deserialize,Serialize};
use super::MOD_PATH;

const CUST_PATH : &str = "customer";

#[derive(Deserialize,Serialize)]
pub struct Customer {
    href     : String,
    id      : String,
    name    : String,
    pub status : Option<String>,
    pub status_reason : Option<String>,
    pub valid_for   : Option<String>,
}

impl Customer {
    pub fn new(name : String) -> Customer {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{id}",MOD_PATH,CUST_PATH);
        Customer {
            id,
            href,
            name,
            status : None,
            status_reason: None,
            valid_for: None,
        }
    }
}