use super::ExternalIdentifierFvo;
use crate::common::entity::Entity;
use serde::{Deserialize, Serialize};

///PlaceFvo defines a place for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlaceFvo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Collection of external identifiers
    #[serde(rename = "externalIdentifier")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_identifier: Vec<ExternalIdentifierFvo>,
}
impl std::fmt::Display for PlaceFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PlaceFvo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for PlaceFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
