use serde::{Serialize, Deserialize};
use super::{ ProductOfferingRef, ProductStatusType};
use crate::common::entity::Entity;

/// Customer360 Product VO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer360ProductVo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Is the description of the product. It could be copied from the description of the Product Offering.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Name of the product. It could be the same as the name of the product offering
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Reference to the product offering from which the product is instantiated
    #[serde(rename = "productOffering")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_offering: Option<ProductOfferingRef>,
    ///Possible values for the status of the product
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ProductStatusType>,
}
impl std::fmt::Display for Customer360ProductVo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Customer360ProductVo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Customer360ProductVo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
