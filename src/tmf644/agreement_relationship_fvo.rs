use serde::{Serialize, Deserialize};
use super::{EntityRefFvo};
use crate::TimePeriod;
// use crate::common::extensible::Extensible;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgreementRelationshipFvo {
    #[serde(flatten)]
    pub entity_ref_fvo: EntityRefFvo,
    ///Unique identifier of the related agreement
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Name of the agreement
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Type of relationship such as containment, substitution, dependency, exclusivity
    #[serde(rename = "relationshipType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for AgreementRelationshipFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AgreementRelationshipFvo {
    type Target = EntityRefFvo;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref_fvo
    }
}
impl std::ops::DerefMut for AgreementRelationshipFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref_fvo
    }
}
