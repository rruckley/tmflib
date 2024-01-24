//! Product Module
//!
use serde::{Deserialize, Serialize};

use crate::{CreateTMF, HasId, HasName, LIB_PATH, DateTime};
use tmflib_derive::{HasId,HasName};

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
#[derive(Debug, Default, Deserialize, HasId, HasName, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_bundle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_customer_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_date: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_serial_number: Option<String>,
    start_date: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    termination_date: Option<DateTime>,
    status: ProductStatusType,
}

impl Product {
    /// Create a new product object
    pub fn new(name: impl Into<String>) -> Product {
        let mut product = Product::create();
        product.status = ProductStatusType::Created;
        product.name = Some(name.into());
        product
    }
}
