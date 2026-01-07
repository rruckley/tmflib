use serde::{Serialize, Deserialize};
use super::EntityRef;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgreementSpecificationRef {
    #[serde(flatten)]
    pub entity_ref: EntityRef,
    ///A narrative that explains in detail what the agreement specification is about.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Agreement specification version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for AgreementSpecificationRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AgreementSpecificationRef {
    type Target = EntityRef;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref
    }
}
impl std::ops::DerefMut for AgreementSpecificationRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref
    }
}
