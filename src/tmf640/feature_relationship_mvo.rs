use serde::{Serialize, Deserialize};
use super::{EntityRefMvo};
use crate::TimePeriod;

///FeatureRelationshipMvo
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureRelationshipMvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub entity_ref_mvo: EntityRefMvo,
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
impl std::fmt::Display for FeatureRelationshipMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for FeatureRelationshipMvo {
    type Target = EntityRefMvo;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref_mvo
    }
}
impl std::ops::DerefMut for FeatureRelationshipMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref_mvo
    }
}
