use serde::{Serialize, Deserialize};
use super::{Any, CharacteristicRelationship};
///Describes a given characteristic of an object or entity through a name/value pair.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Characteristic {
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
    ///This is a list of characteristic relationships.
    #[serde(rename = "characteristicRelationship")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub characteristic_relationship: Option<Vec<CharacteristicRelationship>>,
    ///Unique identifier of the characteristic
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Name of the characteristic
    pub name: String,
    ///Value of the characteristic
    pub value: Any,
    ///Data type of the value of the characteristic
    #[serde(rename = "valueType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}
impl std::fmt::Display for Characteristic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
