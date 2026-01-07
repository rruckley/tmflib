use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceOrderItemRef {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///The actual type of the target instance when needed for disambiguation.
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
    ///Identifier of the line item
    #[serde(rename = "itemId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    ///Link to the order to which this item belongs to
    #[serde(rename = "serviceOrderHref")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_order_href: Option<String>,
    ///Identifier of the order that this item belongs to
    #[serde(rename = "serviceOrderId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_order_id: Option<String>,
}
impl std::fmt::Display for ServiceOrderItemRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
