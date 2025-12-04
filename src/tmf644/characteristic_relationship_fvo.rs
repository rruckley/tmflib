use serde::{Serialize, Deserialize};
use super::ExtensibleFvo;

/// A relationship between characteristics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacteristicRelationshipFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    ///Unique identifier of the characteristic
    pub id: String,
    ///The type of relationship
    #[serde(rename = "relationshipType")]
    pub relationship_type: String,
}
impl std::fmt::Display for CharacteristicRelationshipFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for CharacteristicRelationshipFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for CharacteristicRelationshipFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
