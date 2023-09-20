//! Product Order Module

use super::MOD_PATH;
use uuid::Uuid;
use serde::{Deserialize,Serialize};

const PO_PATH : &str = "order";

#[derive(Debug,Deserialize,Serialize)]
pub struct ProductOrder {
    id : String,
    href : String,
}

impl ProductOrder {
    pub fn new() -> ProductOrder {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}",MOD_PATH,PO_PATH,id);
        ProductOrder {
            id,
            href,
        }
    }
}