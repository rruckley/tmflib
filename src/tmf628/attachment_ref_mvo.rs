use super::EntityRefMvo;
use serde::{Deserialize, Serialize};

///Attachment Reference MVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttachmentRefMvo {
    ///Base Entity Reference MVO
    #[serde(flatten)]
    pub entity_ref_mvo: EntityRefMvo,
    ///A narrative text describing the content of the attachment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Link to the attachment media/content
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl std::fmt::Display for AttachmentRefMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AttachmentRefMvo {
    type Target = EntityRefMvo;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref_mvo
    }
}
impl std::ops::DerefMut for AttachmentRefMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref_mvo
    }
}
