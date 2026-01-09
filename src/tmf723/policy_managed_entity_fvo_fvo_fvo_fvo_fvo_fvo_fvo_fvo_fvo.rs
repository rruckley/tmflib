use serde::{Serialize, Deserialize};
use super::{EntityFvo, NoteFvo};
use crate::TimePeriod;

/// PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo represents a managed entity for policies with various attributes.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity_fvo: EntityFvo,
    ///Description of the Policy Entity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Lifecycle state of the Policy Entity
    #[serde(rename = "lifecycleState")]
    pub lifecycle_state: String,
    ///Name of the Policy Entity
    pub name: String,
    ///Comments related to Policy Entity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<NoteFvo>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    ///Version of the Policy Entity
    pub version: String,
}
impl std::fmt::Display for PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo {
    type Target = EntityFvo;
    fn deref(&self) -> &Self::Target {
        &self.entity_fvo
    }
}
impl std::ops::DerefMut for PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_fvo
    }
}
