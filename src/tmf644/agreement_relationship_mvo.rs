use serde::{Serialize, Deserialize};
use super::{EntityRef};
use crate::TimePeriod;
// use crate::common::extensible::Extensible;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgreementRelationshipMvo {
    #[serde(flatten)]
    pub entity_ref: EntityRef,
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
impl std::fmt::Display for AgreementRelationshipMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AgreementRelationshipMvo {
    type Target = EntityRef;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref
    }
}
impl std::ops::DerefMut for AgreementRelationshipMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref
    }
}
