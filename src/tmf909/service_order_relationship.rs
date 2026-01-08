use serde::{Serialize, Deserialize};
///Linked service order to the one containing this attribute
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceOrderRelationship {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///The entity type of the related order
    #[serde(rename = "@referredType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referred_type: Option<String>,
    ///A URI to a JSON-Schema file that defines additional attributes and relationships
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///When sub-classing, this defines the sub-class Extensible name
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///A hyperlink to the related order
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///The id of the related order
    pub id: String,
    ///The type of related order, such as: [dependency] if the order needs to be [not started] until another order item is complete (a service order in this case) or [cross-ref] to keep track of the source order (a productOrder)
    #[serde(rename = "relationshipType")]
    pub relationship_type: String,
}
impl std::fmt::Display for ServiceOrderRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
