use serde::{Serialize, Deserialize};
use super::{EntityMvo, NoteMvo};
use crate::TimePeriod;

///Policy Managed Entity MVO attributes
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyManagedEntityMvo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity_mvo: EntityMvo,
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
    pub note: Vec<NoteMvo>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    ///Version of the Policy Entity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for PolicyManagedEntityMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PolicyManagedEntityMvo {
    type Target = EntityMvo;
    fn deref(&self) -> &Self::Target {
        &self.entity_mvo
    }
}
impl std::ops::DerefMut for PolicyManagedEntityMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_mvo
    }
}
