use serde::{Serialize, Deserialize};

use crate::common::entity_ref::EntityRef;

/// Product Offering Reference MVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductOfferingRefMvo {
    ///Base Entity Reference schema for use in TMForum Open-APIs
    #[serde(flatten)]
    pub entity_ref: EntityRef,
    ///Version of the product offering
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for ProductOfferingRefMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ProductOfferingRefMvo {
    type Target = EntityRef;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref
    }
}
impl std::ops::DerefMut for ProductOfferingRefMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref
    }
}
