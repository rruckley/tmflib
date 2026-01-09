use serde::{Serialize, Deserialize};
use crate::TimePeriod;
use crate::common::note::Note;
use crate::common::entity::Entity;

/// PolicyManagedEntity represents a managed entity for policies.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyManagedEntity {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Description of the Policy Entity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Lifecycle state of the Policy Entity
    #[serde(rename = "lifecycleState")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<String>,
    ///Name of the Policy Entity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Comments related to Policy Entity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    ///Version of the Policy Entity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for PolicyManagedEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PolicyManagedEntity {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for PolicyManagedEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
