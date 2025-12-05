use serde::{Serialize, Deserialize};
use super::{CharacteristicRelationshipMvo};
use crate::common::extensible::Extensible;

///Characteristic MVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacteristicMvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Collection of characteristic relationships
    #[serde(rename = "characteristicRelationship")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic_relationship: Vec<CharacteristicRelationshipMvo>,
    ///Unique identifier of the characteristic
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Name of the characteristic
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Data type of the value of the characteristic
    #[serde(rename = "valueType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}
impl std::fmt::Display for CharacteristicMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for CharacteristicMvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for CharacteristicMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
