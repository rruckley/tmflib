use super::EntityRef;
use serde::{Deserialize, Serialize};

///Attachment Reference
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttachmentRef {
    ///Reference to an entity
    #[serde(flatten)]
    pub entity_ref: EntityRef,
    ///A narrative text describing the content of the attachment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Link to the attachment media/content
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl std::fmt::Display for AttachmentRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AttachmentRef {
    type Target = EntityRef;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref
    }
}
impl std::ops::DerefMut for AttachmentRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref
    }
}
