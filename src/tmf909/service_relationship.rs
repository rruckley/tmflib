use serde::{Serialize, Deserialize};
use super::{Characteristic, ServiceRefOrValue};

///Relationship between a service and another service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRelationship {
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
    ///unique identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "relationshipType")]
    pub relationship_type: String,
    ///A Service to be created defined by value or existing defined by reference. The polymorphic attributes @type, @schemaLocation & @referredType are related to the Service entity and not the RelatedServiceRefOrValue class itself
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceRefOrValue>,
    #[serde(rename = "serviceRelationshipCharacteristic")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_relationship_characteristic: Option<Vec<Characteristic>>,
}
impl std::fmt::Display for ServiceRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
