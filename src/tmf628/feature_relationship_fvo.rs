use serde::{Serialize, Deserialize};
use super::{EntityRefFvo};
use crate::TimePeriod;

///Feature Relationship FVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureRelationshipFvo {
    ///Base Entity Ref FVO schema
    #[serde(flatten)]
    pub entity_ref_fvo: EntityRefFvo,
    ///This is the name of the target feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///This is the type of the feature relationship.
    #[serde(rename = "relationshipType")]
    pub relationship_type: String,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for FeatureRelationshipFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for FeatureRelationshipFvo {
    type Target = EntityRefFvo;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref_fvo
    }
}
impl std::ops::DerefMut for FeatureRelationshipFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref_fvo
    }
}
