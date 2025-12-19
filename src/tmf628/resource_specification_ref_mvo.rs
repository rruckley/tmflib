use super::EntityRefMvo;
use serde::{Deserialize, Serialize};

///Resource Specification Reference
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceSpecificationRefMvo {
    ///Entity reference MVO schema for use in TMForum Open-APIs. Property used to extend other entities.
    #[serde(flatten)]
    pub entity_ref_mvo: EntityRefMvo,
    ///Resource Specification version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for ResourceSpecificationRefMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ResourceSpecificationRefMvo {
    type Target = EntityRefMvo;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref_mvo
    }
}
impl std::ops::DerefMut for ResourceSpecificationRefMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref_mvo
    }
}
