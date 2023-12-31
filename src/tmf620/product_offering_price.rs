//! Product Offering Price Module

use serde::{Deserialize,Serialize};

use super::MOD_PATH;
use crate::{HasId,HasName, CreateTMF, HasLastUpdate, CreateTMFWithTime, LIB_PATH, TimePeriod};
use crate::common::money::Money;
const PRICE_PATH : &str = "productOfferingPrice";
const PRICE_VERS : &str = "1.0";

/// Constraints
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstraintRef {
    /// Unique Id
    id: String,
    /// HTTP Reference
    href: String,
    /// Name
    name: String,
    /// Version
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
}

/// Tax Details
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxItem {
    tax_category: String,
    tax_rate: f32,
    tax_amount: Money,
}

/// Product Offering Price Reference
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductOfferingPriceRef {
    /// Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTTP Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Name
    pub name: String,
}

impl From<ProductOfferingPrice> for ProductOfferingPriceRef {
    fn from(pop : ProductOfferingPrice) -> ProductOfferingPriceRef {
        ProductOfferingPriceRef { 
            id: pop.id.clone(), 
            href: pop.href.clone(), 
            name: pop.name.as_ref().unwrap().clone(),
        ..Default::default()
        }
    }
}

/// Pricing linked to a Product Offering
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductOfferingPrice {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTML Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Versoin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    constraint: Option<Vec<ConstraintRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_bundle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_update: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring_charge_period_length: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring_charge_period_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_of_measure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax: Option<Vec<TaxItem>>,
}

impl ProductOfferingPrice {
    /// Create a new Price Offering Price object
    pub fn new(name :  String) -> ProductOfferingPrice {
        let mut pop = ProductOfferingPrice::create_with_time();
        pop.version = Some(PRICE_VERS.to_string());
        pop.name = Some(name);
        pop
    }
}
impl CreateTMF<ProductOfferingPrice> for ProductOfferingPrice {}
impl CreateTMFWithTime<ProductOfferingPrice> for ProductOfferingPrice {}

impl HasId for ProductOfferingPrice {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,PRICE_PATH,self.get_id());
        self.href = Some(href);
    }
    fn generate_id(&mut self) {
        let id = ProductOfferingPrice::get_uuid();
        self.id = Some(id);
        self.generate_href();
    }
    fn get_href(&self) -> String {
        self.href.as_ref().unwrap().clone()
    }
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone()
    }

    fn get_class() -> String {
        PRICE_PATH.to_owned()
    }
}

impl HasName for ProductOfferingPrice {
    fn get_name(&self) -> String {
        self.name.as_ref().unwrap().clone()
    }
}

impl HasLastUpdate for ProductOfferingPrice {
    fn set_last_update(&mut self, time : String) {
        self.last_update = Some(time);
    }
}