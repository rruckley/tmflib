//! Product Offering Price Module

use serde::{Deserialize,Serialize};
use uuid::Uuid;

use super::LIB_PATH;
use super::MOD_PATH;
const PRICE_PATH : &str = "price";
const PRICE_VERS : &str = "1.0";

/// Product Offering Price Reference
#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductOfferingPrice {
    id: Option<String>,
    href: Option<String>,
    name: String,
    version: String,
}

impl ProductOfferingPrice {
    pub fn new(name :  String) -> ProductOfferingPrice {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,PRICE_PATH,id);
        ProductOfferingPrice { 
            id: Some(id), 
            href: Some(href), 
            name, 
            version: PRICE_VERS.to_string() 
        }
    }
}