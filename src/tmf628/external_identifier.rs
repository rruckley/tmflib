use serde::{Serialize, Deserialize};
use crate::common::extensible::Extensible;

///External Identifier
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalIdentifier {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Type of the identification, typically would be the type of the entity within the external system
    #[serde(rename = "externalIdentifierType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_identifier_type: Option<String>,
    ///identification of the entity within the external system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Name of the external system that owns the entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}
impl std::fmt::Display for ExternalIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ExternalIdentifier {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for ExternalIdentifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
