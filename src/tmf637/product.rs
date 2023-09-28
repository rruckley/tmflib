//! Product Module
//!
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::LIB_PATH;
use super::MOD_PATH;

const PROD_PATH: &str = "product";

#[derive(Debug, Default, Deserialize, Serialize)]
enum ProductStatusType {
    #[default]
    Created,
    Cancelled,
    Active,
    PendingActive,
    PendingTerminate,
    Terminated,
    Suspended,
    Aborted,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Product {
    id: String,
    href: String,
    description: Option<String>,
    name: String,
    status: ProductStatusType,
}

impl Product {
    pub fn new(name: String) -> Product {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}", LIB_PATH, MOD_PATH, PROD_PATH, id);
        Product {
            id,
            href,
            description: None,
            name,
            status: ProductStatusType::Created,
        }
    }
}
