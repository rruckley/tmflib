//! Entity Module
//! Defines the base Entity struct used across TMForum Open-APIs

use serde::{Serialize, Deserialize};
use crate::Uri;
///Base entity schema for use in TMForum Open-APIs. Property.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Entity {
    /// Unique identifier of the entity
    pub id : Option<String>,
    /// Hyperlink to the entity
    pub href : Option<Uri>,
}
impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

///Entity Ref MVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityRef {
    ///The actual type of the target instance when needed for disambiguation.
    #[serde(rename = "@referredType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referred_type: Option<String>,
    ///Base schema for addressable entities
    #[serde(flatten)]
    pub addressable: Addressable,
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///The URI of the referred entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///The identifier of the referred entity.
    pub id: String,
    ///Name of the referred entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for EntityRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl std::ops::Deref for EntityRef {
    type Target = Addressable;
    fn deref(&self) -> &Self::Target {
        &self.addressable
    }
}

impl std::ops::DerefMut for EntityRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.addressable
    }
}
