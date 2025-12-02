use serde::{Serialize, Deserialize};
use super::{DocumentRefOrValueMvo, Extensible};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelatedDocumentRefOrValueMvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<DocumentRefOrValueMvo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
impl std::fmt::Display for RelatedDocumentRefOrValueMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for RelatedDocumentRefOrValueMvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for RelatedDocumentRefOrValueMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
