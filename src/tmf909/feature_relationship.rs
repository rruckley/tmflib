use serde::{Serialize, Deserialize};
use crate::TimePeriod;
///Configuration feature
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureRelationship {
    ///Unique identifier of the target feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///This is the name of the target feature.
    pub name: String,
    ///This is the type of the feature relationship.
    #[serde(rename = "relationshipType")]
    pub relationship_type: String,
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
