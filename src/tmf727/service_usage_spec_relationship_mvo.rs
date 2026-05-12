use serde::{Serialize, Deserialize};
use super::EntityRefMvo;
use crate::TimePeriod;

/// A migration, substitution, dependency or exclusivity relationship between/among service usage specifications.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceUsageSpecRelationshipMvo {
    #[serde(flatten)]
    pub entity_ref_mvo: EntityRefMvo,
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
impl std::fmt::Display for ServiceUsageSpecRelationshipMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ServiceUsageSpecRelationshipMvo {
    type Target = EntityRefMvo;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref_mvo
    }
}
impl std::ops::DerefMut for ServiceUsageSpecRelationshipMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref_mvo
    }
}
