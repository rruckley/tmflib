//! Product Module
//!
use serde::{Deserialize, Serialize};

use crate::tmf651::agreement::AgreementRef;
use crate::tmf666::billing_account::BillingAccountRef;
use crate::{CreateTMF, HasId, HasName, LIB_PATH, DateTime,HasValidity, TimePeriod};
use tmflib_derive::{HasId, HasName, HasValidity};
use crate::common::related_place::RelatedPlaceRefOrValue;
#[cfg(feature = "tmf620-v4")]
use crate::tmf620::product_offering::ProductOfferingRef;
#[cfg(feature = "tmf620-v5")]
use crate::tmf620::product_offering_v5::ProductOfferingRef;

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
    // References
    #[serde(skip_serializing_if = "Option::is_none")]
    agreement: Option<Vec<AgreementRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    place: Option<Vec<RelatedPlaceRefOrValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_offering: Option<ProductOfferingRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_account: Option<BillingAccountRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_term: Option<Vec<ProductTerm>>,
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

/// Product Term
#[derive(Clone,Debug,Default,Deserialize, HasValidity, Serialize)]
pub struct ProductTerm {
    /// Term Description
    description: Option<String>,
    /// Term Name
    name: Option<String>,
    /// Term duration in days
    duration: u16,
    /// Validity period
    valid_for: Option<TimePeriod>,
}