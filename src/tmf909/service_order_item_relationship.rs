use serde::{Serialize, Deserialize};
use super::ServiceOrderItemRef;
///Linked service order item to the one containing this attribute
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceOrderItemRelationship {
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
    ///Reference to the related order item
    #[serde(rename = "orderItem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_item: Option<ServiceOrderItemRef>,
    ///The type of related order item, can be: dependency if the order item needs to be not started until another order item is complete
    #[serde(rename = "relationshipType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
}
impl std::fmt::Display for ServiceOrderItemRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
