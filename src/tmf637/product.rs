//! Product Module
//!
use serde::{Deserialize, Serialize};

use crate::CreateTMF;
use crate::HasId;

use super::LIB_PATH;
use super::MOD_PATH;

const PROD_PATH: &str = "product";

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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

/// Product record from the Product Inventory
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    id: Option<String>,
    href: Option<String>,
    description: Option<String>,
    name: String,
    status: ProductStatusType,
}

impl HasId for Product {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}", LIB_PATH, MOD_PATH, PROD_PATH, self.get_id());
        self.href = Some(href);    
    }
    fn generate_id(&mut self) {
        let id = Product::get_uuid();
        self.id = Some(id);
        self.generate_href();
    }
    fn get_href(&self) -> String {
        self.href.as_ref().unwrap().clone()    
    }
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone()    
    }
}
impl CreateTMF<Product> for Product {}

impl Product {
    /// Create a new product object
    pub fn new(name: String) -> Product {
        let mut product = Product::create();
        product.status = ProductStatusType::Created;
        product.name = name;
        product
    }
}
