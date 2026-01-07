use serde::{Serialize, Deserialize};
///Structure used to describe relationship between serviceQualification items from the same serviceQualification.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceQualificationItemRelationship {
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
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///Id of the serviceQualificationItem (from the same serviceQualificationItem).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Type of relationship (ex: reliesOn, connectedTo, etc...)
    #[serde(rename = "relationshipType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
}
impl std::fmt::Display for ServiceQualificationItemRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
