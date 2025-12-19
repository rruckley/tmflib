use super::Entity;
use crate::TimePeriod;
use serde::{Deserialize, Serialize};

///Performance Indicator Spec Relationship MVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceIndicatorSpecRelationshipMvo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    /// The type of relationship such as 'dependsOn', 'relatesTo', 'isComposedOf'
    #[serde(rename = "relationshipType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
    ///The association role for this service specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for PerformanceIndicatorSpecRelationshipMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PerformanceIndicatorSpecRelationshipMvo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for PerformanceIndicatorSpecRelationshipMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
