use super::{DocumentRefOrValueFvo, ExtensibleFvo};
use serde::{Deserialize, Serialize};

/// Related Document Reference or Value FVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelatedDocumentRefOrValueFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    ///Reference or value of the related document
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<DocumentRefOrValueFvo>,
    ///Role of the related document
    pub role: String,
}
impl std::fmt::Display for RelatedDocumentRefOrValueFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for RelatedDocumentRefOrValueFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for RelatedDocumentRefOrValueFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
