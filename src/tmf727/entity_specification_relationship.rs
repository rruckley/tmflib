use serde::{Serialize, Deserialize};
use super::{AssociationSpecificationRef, TimePeriod};
///A migration, substitution, dependency or exclusivity relationship between/among entity specifications.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntitySpecificationRelationship {
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///reference to an AssociationSpecification object
    #[serde(rename = "associationSpec")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub association_spec: Option<AssociationSpecificationRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Type of relationship such as migration, substitution, dependency, exclusivity
    #[serde(rename = "relationshipType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
    ///The association role for this entity specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for EntitySpecificationRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
