//! Product Module
//!
use serde::{Deserialize, Serialize};

use crate::{CreateTMF, HasId};
use tmflib_derive::HasId;

use crate::LIB_PATH;
use super::MOD_PATH;

const CLASS_PATH: &str = "product";

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
#[derive(Debug, Default, Deserialize,HasId, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    id: Option<String>,
    href: Option<String>,
    description: Option<String>,
    name: String,
    status: ProductStatusType,
}

impl Product {
    /// Create a new product object
    pub fn new(name: impl Into<String>) -> Product {
        let mut product = Product::create();
        product.status = ProductStatusType::Created;
        product.name = name.into();
        product
    }
}
