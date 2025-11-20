use serde::{Serialize, Deserialize};
use super::EntityRefFvo;

///Resource Specification Reference
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceSpecificationRefFvo {
    ///Base Entity Reference schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub entity_ref_fvo: EntityRefFvo,
    ///Resource Specification version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for ResourceSpecificationRefFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ResourceSpecificationRefFvo {
    type Target = EntityRefFvo;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref_fvo
    }
}
impl std::ops::DerefMut for ResourceSpecificationRefFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref_fvo
    }
}
