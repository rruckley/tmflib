use serde::{Deserialize, Serialize};
// use super::{EntityRef, TimePeriod};
use crate::common::entity::EntityRef;
use crate::TimePeriod;

/// A migration, substitution, dependency or exclusivity relationship between/among service usage specifications.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceUsageSpecRelationship {
    ///Base entity schema for use in `TMForum` Open-APIs. Property.
    #[serde(flatten)]
    pub entity_ref: EntityRef,
    ///Type of relationship such as dependency, substitution or exclusivity
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
impl std::fmt::Display for ServiceUsageSpecRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ServiceUsageSpecRelationship {
    type Target = EntityRef;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref
    }
}
impl std::ops::DerefMut for ServiceUsageSpecRelationship {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref
    }
}
