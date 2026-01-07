use serde::{Serialize, Deserialize};
use crate::common::entity::Entity;
use crate::TimePeriod;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Customer360PartyInteractionVo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Description of the interaction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "interactionDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction_date: Option<TimePeriod>,
    ///Reason why the interaction happened
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    ///Status of the interaction (opened, inProgress, completed)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for Customer360PartyInteractionVo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Customer360PartyInteractionVo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Customer360PartyInteractionVo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
