use serde::{Serialize, Deserialize};
use super::{DocumentRefOrValue, Extensible};

/// Related Document Reference or Value
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelatedDocumentRefOrValue {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<DocumentRefOrValue>,
    ///Role of the related document
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
impl std::fmt::Display for RelatedDocumentRefOrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for RelatedDocumentRefOrValue {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for RelatedDocumentRefOrValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
