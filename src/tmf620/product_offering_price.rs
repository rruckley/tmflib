//! Product Offering Price Module

use serde::{Deserialize,Serialize};

use super::MOD_PATH;
use crate::{HasId,CreateTMF, LIB_PATH};
const PRICE_PATH : &str = "price";
const PRICE_VERS : &str = "1.0";

/// Product Offering Price Reference
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ProductOfferingPriceRef {
    id: String,
    href: String,
    name: String,
}

impl From<ProductOfferingPrice> for ProductOfferingPriceRef {
    fn from(pop : ProductOfferingPrice) -> ProductOfferingPriceRef {
        ProductOfferingPriceRef { id: pop.id.unwrap().clone(), href: pop.href.unwrap().clone(), name: pop.name.clone() }
    }
}

/// Pricing linked to a Product Offering
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ProductOfferingPrice {
    id: Option<String>,
    href: Option<String>,
    name: String,
    version: String,
}

impl ProductOfferingPrice {
    /// Create a new Price Offering Price object
    pub fn new(name :  String) -> ProductOfferingPrice {
        let mut pop = ProductOfferingPrice::create();
        pop.version = PRICE_VERS.to_string();
        pop.name = name;
        pop
    }
}
impl CreateTMF<ProductOfferingPrice> for ProductOfferingPrice {}

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
}