use super::EntityRefFvo;
use serde::{Deserialize, Serialize};

/// Product Offering Reference FVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductOfferingRefFvo {
    ///Base Entity Reference FVO schema for use in TMForum Open-APIs
    #[serde(flatten)]
    pub entity_ref_fvo: EntityRefFvo,
    ///Version of the product offering
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for ProductOfferingRefFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ProductOfferingRefFvo {
    type Target = EntityRefFvo;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref_fvo
    }
}
impl std::ops::DerefMut for ProductOfferingRefFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref_fvo
    }
}
