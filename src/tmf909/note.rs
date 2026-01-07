use serde::{Serialize, Deserialize};
///Extra information about a given entity
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Note {
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
    ///Author of the note
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    ///Date of the note
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::DateTime>,
    ///Identifier of the note within its containing entity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Text of the note
    pub text: String,
}
impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
