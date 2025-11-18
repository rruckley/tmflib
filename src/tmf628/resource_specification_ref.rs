use serde::{Serialize, Deserialize};
use super::EntityRef;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceSpecificationRef {
    #[serde(flatten)]
    pub entity_ref: EntityRef,
    ///Resource Specification version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for ResourceSpecificationRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ResourceSpecificationRef {
    type Target = EntityRef;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref
    }
}
impl std::ops::DerefMut for ResourceSpecificationRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref
    }
}
