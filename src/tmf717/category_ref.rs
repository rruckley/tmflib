use serde::{Serialize, Deserialize};
use super::EntityRef;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CategoryRef {
    #[serde(flatten)]
    pub entity_ref: EntityRef,
    ///Version of the category
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for CategoryRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for CategoryRef {
    type Target = EntityRef;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref
    }
}
impl std::ops::DerefMut for CategoryRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref
    }
}
