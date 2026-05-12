use serde::{Serialize, Deserialize};
use super::{AssociationSpecificationRefFvo, TimePeriod};
///A migration, substitution, dependency or exclusivity relationship between/among entity specifications.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntitySpecificationRelationshipFvo {
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///reference to an AssociationSpecification object
    #[serde(rename = "associationSpec")]
    pub association_spec: AssociationSpecificationRefFvo,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Type of relationship such as migration, substitution, dependency, exclusivity
    #[serde(rename = "relationshipType")]
    pub relationship_type: String,
    ///The association role for this entity specification
    pub role: String,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    pub valid_for: TimePeriod,
}
impl std::fmt::Display for EntitySpecificationRelationshipFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
