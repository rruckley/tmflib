//! Product Module
//!
use serde::{Deserialize, Serialize};

use crate::tmf651::agreement::AgreementRef;
use crate::tmf666::billing_account::BillingAccountRef;
use crate::{HasId, HasName, LIB_PATH, DateTime,HasValidity, TimePeriod};
use tmflib_derive::{HasId, HasName, HasValidity};
use crate::common::related_place::RelatedPlaceRefOrValue;
use crate::common::product::ProductStatusType;
use crate::tmf620::product_offering_price::ProductOfferingPriceRef;
use crate::common::price::Price;
#[cfg(feature = "tmf620-v4")]
use crate::tmf620::product_offering::ProductOfferingRef;
#[cfg(feature = "tmf620-v5")]
use crate::tmf620::product_offering_v5::ProductOfferingRef;

use super::MOD_PATH;

const CLASS_PATH: &str = "product";

/// Product Price information for recurring charges
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductPrice {
    #[serde(skip_serializing_if = "Option::is_none")]
    description : Option<String>,
    name: String,
    price_type: String,
    recurring_charge_period: String,
    unit_of_measure: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_offering_price: Option<ProductOfferingPriceRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_price_alteration: Option<Vec<PriceAlteration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_account: Option<BillingAccountRef>,
}

/// Pricing alteration 
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceAlteration {
    application_duration: u32,
    description: String,
    name : String,
    price_type: String,
    priority: i16,
    recurring_charge_period:  String,
    unit_of_measure: String,
    price: Option<Price>,
    product_offering_price: Option<ProductOfferingPriceRef>,
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
    // Referenced types
    #[serde(skip_serializing_if = "Option::is_none")]
    product_price : Option<Vec<ProductPrice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_account: Option<BillingAccountRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_offering: Option<ProductOfferingRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_term: Option<Vec<ProductTerm>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agreement: Option<Vec<AgreementRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    place: Option<Vec<RelatedPlaceRefOrValue>>,
}

impl Product {
    /// Create a new product object
    pub fn new(name: impl Into<String>) -> Product {
        let mut product = Product::create();
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::product::ProductStatusType;

    const PRODUCT : &str = "AProduct";

    #[test]
    fn test_product_new_name() {
        let product = Product::new(PRODUCT);

        assert_eq!(product.name,Some(PRODUCT.into()));
    }

    #[test]
    fn test_product_new_status() {
        let product = Product::new(PRODUCT);

        assert_eq!(product.status,ProductStatusType::Created);
    }
}