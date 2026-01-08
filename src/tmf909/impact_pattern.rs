use serde::{Serialize, Deserialize};
use super::Characteristic;
///Define the patterns of impact (optional), such as other service characteristics- Used when defining impact through another pattern than the pre-defined attributes
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImpactPattern {
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
    ///A generic list of any type of elements. Used for extensions or loose element encapsulation from other namespaces
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub characteristic: Option<Vec<Characteristic>>,
    ///Basic description of the impact pattern
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///unique identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl std::fmt::Display for ImpactPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
