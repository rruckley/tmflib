use serde::{Serialize, Deserialize};
use crate::common::entity::EntityRef;

/// Product Offering Reference
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductOfferingRef {
    /// Base entity reference schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity_ref: EntityRef,
    ///Version of the product offering
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for ProductOfferingRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ProductOfferingRef {
    type Target = EntityRef;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref
    }
}
impl std::ops::DerefMut for ProductOfferingRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref
    }
}
