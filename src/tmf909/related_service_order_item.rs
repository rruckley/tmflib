use serde::{Serialize, Deserialize};
use super::OrderItemActionType;
///RelatedServiceOrderItem (a ServiceOrder item) .The service order item which triggered service creation/change/termination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedServiceOrderItem {
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
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///unique identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///action to be performed on the product
    #[serde(rename = "itemAction")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_action: Option<OrderItemActionType>,
    ///Identifier of the order item where the service was managed
    #[serde(rename = "itemId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    ///role of the service order item for this service
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    ///Reference of the related entity.
    #[serde(rename = "serviceOrderHref")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_order_href: Option<String>,
    ///Unique identifier of a related entity.
    #[serde(rename = "serviceOrderId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_order_id: Option<String>,
}
impl std::fmt::Display for RelatedServiceOrderItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
