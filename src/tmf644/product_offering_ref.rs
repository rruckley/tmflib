use serde::{Deserialize, Serialize};

use crate::common::entity_ref::EntityRef;

/// Reference to a ProductOffering
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductOfferingRef {
    /// Entity reference
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
