//! EntityRef Module, references to an Entity
//! Defines the base EntityRef struct used across TMForum Open-APIs

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
