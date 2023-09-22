//! Product Module
//! 
use uuid::Uuid;
use serde::{Deserialize,Serialize};

use super::MOD_PATH;

const PROD_PATH : &str = "product";

#[derive(Debug,Deserialize,Serialize)]
enum ProductStatusType {
    Created,
    Cancelled,
    Active,
    PendingActive,
    PendingTerminate,
    Terminated,
    Suspended,
    Aborted,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Product {
    id      : String,
    href    : String,
    description : Option<String>,
    name    : String,
    status  : ProductStatusType,
}

impl Product {
    pub fn new(name : String) -> Product {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}",MOD_PATH,PROD_PATH,id);
        Product {
            id,
            href,
            description : None,
            name,
            status : ProductStatusType::Created,
        }
    }
}