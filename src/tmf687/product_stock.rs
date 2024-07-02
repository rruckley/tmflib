//! Product Stock Module


use super::MOD_PATH;
use crate::{
    LIB_PATH,
    HasId,
    HasName,
    HasLastUpdate,
    HasRelatedParty,
    Uri,
    DateTime,
};
use tmflib_derive::{
    HasId,
    HasName,
    HasLastUpdate,
    HasRelatedParty,
};
use crate::common::related_party::RelatedParty;
use crate::common::related_place::RelatedPlaceRefOrValue;
use crate::common::product::ProductRefOrValue;
use serde::{Deserialize,Serialize};

const CLASS_PATH : &str = "productStock";

#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct ProductStockRelationship {
    relationship_type : String,

}

#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct ProductStockRef {
    href: Uri,
    id: String,
    name: String,
}

impl From<ProductStock> for ProductStockRef {
    fn from(value: ProductStock) -> Self {
        ProductStockRef {
            id: value.get_id(),
            href: value.get_href(),
            name: value.get_name(),
        }
    }
}

/// Product Stock Record
#[derive(Clone,Default,Debug,Deserialize,HasId,HasName,HasLastUpdate,HasRelatedParty,Serialize)]
pub struct ProductStock {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<Uri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_update: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_party: Option<Vec<RelatedParty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    place: Option<Vec<RelatedPlaceRefOrValue>>,
    stocked_product: ProductRefOrValue,
    product_stock_relationship: Option<Vec<ProductStockRelationship>>,
}

impl ProductStock {
    /// Create a new ProductStock instance
    pub fn new(name : impl Into<String>) -> ProductStock {
        ProductStock {
            name: Some(name.into()),
            ..ProductStock::create()
        }
    }
}