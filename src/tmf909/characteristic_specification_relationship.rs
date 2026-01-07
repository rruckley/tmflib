use serde::{Serialize, Deserialize};
use crate::TimePeriod;
///An aggregation, migration, substitution, dependency or exclusivity relationship between/among Characteristic specifications. The specification characteristic is embedded within the specification whose ID and href are in this entity, and identified by its ID.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacteristicSpecificationRelationship {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///A URI to a JSON-Schema file that defines additional attributes and relationships
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///When sub-classing, this defines the sub-class Extensible name
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///Unique identifier of the characteristic within the specification
    #[serde(rename = "characteristicSpecificationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub characteristic_specification_id: Option<String>,
    ///Name of the target characteristic within the specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Hyperlink reference to the parent specification containing the target characteristic
    #[serde(rename = "parentSpecificationHref")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_specification_href: Option<String>,
    ///Unique identifier of the parent specification containing the target characteristic
    #[serde(rename = "parentSpecificationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_specification_id: Option<String>,
    ///Type of relationship such as aggregation, migration, substitution, dependency, exclusivity
    #[serde(rename = "relationshipType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for CharacteristicSpecificationRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
