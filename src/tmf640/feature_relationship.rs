use serde::{Serialize, Deserialize};
use super::{EntityRef};
use crate::TimePeriod;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureRelationship {
    #[serde(flatten)]
    pub entity_ref: EntityRef,
    ///This is the name of the target feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///This is the type of the feature relationship.
    #[serde(rename = "relationshipType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for FeatureRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for FeatureRelationship {
    type Target = EntityRef;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref
    }
}
impl std::ops::DerefMut for FeatureRelationship {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref
    }
}
