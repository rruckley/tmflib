use serde::{Serialize, Deserialize};
use super::{ExtensibleFvo};
use crate::TimePeriod;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacteristicSpecificationRelationshipFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    ///Unique identifier of the characteristic within the specification
    #[serde(rename = "characteristicSpecificationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub characteristic_specification_id: Option<String>,
    ///Name of the target characteristic within the specification
    pub name: String,
    ///Hyperlink reference to the parent specification containing the target characteristic
    #[serde(rename = "parentSpecificationHref")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_specification_href: Option<String>,
    ///Unique identifier of the parent specification containing the target characteristic
    #[serde(rename = "parentSpecificationId")]
    pub parent_specification_id: String,
    ///Type of relationship such as aggregation, migration, substitution, dependency, exclusivity
    #[serde(rename = "relationshipType")]
    pub relationship_type: String,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for CharacteristicSpecificationRelationshipFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for CharacteristicSpecificationRelationshipFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for CharacteristicSpecificationRelationshipFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
