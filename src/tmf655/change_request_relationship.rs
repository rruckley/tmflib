use super::{ChangeRequestRefOrValue, Characteristic};
use serde::{Deserialize, Serialize};

///A relationship between two change requests. The polymorphic attributes @type, @schemaLocation & @referredType are related to the ChangeRequest entity and not the ChangeRequestRelationship class itself
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeRequestRelationship {
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
    ///A Service to be created defined by value or existing defined by reference. The polymorphic attributes @type, @schemaLocation & @referredType are related to the Service entity and not the RelatedServiceRefOrValue class itself
    #[serde(rename = "changeRequest")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_request: Option<ChangeRequestRefOrValue>,
    /// Relationship Characteristics are used to describe the relationship between two change requests. The polymorphic attributes @type, @schemaLocation & @referredType are related to the Characteristic entity and not the ChangeRequestRelationshipCharacteristic class itself
    #[serde(rename = "changeRequestRelationshipCharacteristic")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_request_relationship_characteristic: Option<Vec<Characteristic>>,
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///unique identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///The actual type of the target instance when needed for disambiguation.
    #[serde(rename = "relationshipType")]
    pub relationship_type: String,
}
impl std::fmt::Display for ChangeRequestRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
