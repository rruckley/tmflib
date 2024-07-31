//! Product Module
//!
use serde::{Deserialize, Serialize};

use crate::tmf620::product_specification::ProductSpecificationRef;
use crate::tmf651::agreement::AgreementRef;
// use crate::tmf651::agreement_item::AgreementItemRef;
use crate::tmf666::billing_account::BillingAccountRef;
use crate::{HasId, HasName, LIB_PATH, DateTime,HasValidity, TimePeriod};
use tmflib_derive::{HasId, HasName, HasValidity};
use crate::common::related_place::RelatedPlaceRefOrValue;
use crate::common::product::{ProductRefOrValue,ProductStatusType};
use crate::common::price::Price;
#[cfg(feature = "tmf620-v4")]
use crate::tmf620::product_offering::ProductOfferingRef;
#[cfg(feature = "tmf620-v5")]
use crate::tmf620::product_offering_v5::ProductOfferingRef;
use crate::tmf620::product_offering_price::ProductOfferingPriceRef;

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
    // References
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
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<Vec<ProductRefOrValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_specification: Option<ProductSpecificationRef>,
    // realizing_service: Option<Vec<ServiceRef>>,
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
#[serde(rename_all = "camelCase")]
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
    use crate::{TimePeriod,HasValidity};

    const PRODUCT_NAME : &str = "A Product";

    const PRODUCTPRICE_JSON : &str = "{
        \"applicationDuration\" : 1,
        \"description\" : \"Description\",
        \"name\" : \"Name\",
        \"priceType\" : \"Fixed\",
        \"priority\" : 2,
        \"recurringChargePeriod\" : \"monthly\",
        \"unitOfMeasure\" : \"Dollars\"
    }";
    const PRICEALTERATION_JSON : &str = "{
        \"applicationDuration\" : 1,
        \"description\" : \"Description\",
        \"name\" : \"Name\",
        \"priceType\" : \"Fixed\",
        \"priority\" : 2,
        \"recurringChargePeriod\" : \"monthly\",
        \"unitOfMeasure\" : \"Dollars\"
    }";
    const PRODUCT_JSON : &str = "{
        \"status\" : \"Created\"
    }";
    const PRODUCTTERM_JSON : &str = "{
        \"name\" : \"annual\",
        \"duration\" : 365
    }";

    #[test]
    fn test_product_new_name() {
        let product = Product::new(PRODUCT_NAME);

        assert_eq!(product.get_name(),PRODUCT_NAME);
    }

    #[test]
    fn test_product_new_status() {
        let product = Product::new(PRODUCT_NAME);

        assert_eq!(product.status,ProductStatusType::Created);
    }

    #[test]
    fn test_productprice_deserialize() {
        let productprice : ProductPrice = serde_json::from_str(PRODUCTPRICE_JSON).unwrap();

        assert_eq!(productprice.name.as_str(),"Name");
        assert_eq!(productprice.description.is_some(),true);
        assert_eq!(productprice.description.unwrap().as_str(),"Description");
    }

    #[test]
    fn test_pricealteration_deserialize() {
        let pricealteration : PriceAlteration = serde_json::from_str(PRICEALTERATION_JSON).unwrap();

        assert_eq!(pricealteration.application_duration,1);
        assert_eq!(pricealteration.priority,2);
    }

    #[test]
    fn test_product_deserialize() {
        let product : Product = serde_json::from_str(PRODUCT_JSON).unwrap();

        assert_eq!(product.status,ProductStatusType::Created);
    }

    #[test]
    fn test_product_hasname() {
        let product = Product::new(PRODUCT_NAME);

        assert_eq!(product.get_name().as_str(),PRODUCT_NAME);
    }

    #[test]
    fn test_productterm_desrialize() {
        let productterm : ProductTerm = serde_json::from_str(PRODUCTTERM_JSON).unwrap();

        assert_eq!(productterm.name.is_some(),true);
        assert_eq!(productterm.name.unwrap().as_str(),"annual");
        assert_eq!(productterm.duration,365);
    }

    #[test]
    fn test_productterm_hasvalidity() {
        let mut productterm = ProductTerm::default();
        productterm.set_validity(TimePeriod::period_30days());

        assert_eq!(productterm.valid_for.is_some(),true);
        let validity = productterm.get_validity().unwrap();
        assert_eq!(validity.started(),true);
        assert_eq!(validity.finished(),false);
    }
}