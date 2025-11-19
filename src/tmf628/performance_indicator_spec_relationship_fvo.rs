use serde::{Serialize, Deserialize};
use super::{Entity};
use crate::TimePeriod;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceIndicatorSpecRelationshipFvo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///
    #[serde(rename = "relationshipType")]
    pub relationship_type: String,
    ///The association role for this service specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    pub valid_for: TimePeriod,
}
impl std::fmt::Display for PerformanceIndicatorSpecRelationshipFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PerformanceIndicatorSpecRelationshipFvo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for PerformanceIndicatorSpecRelationshipFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
