use serde::{Serialize, Deserialize};
use super::{OrderItemActionType};
use crate::common::extensible::Extensible  ;

///Related service order item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedServiceOrderItem {
    ///The actual type of the target instance when needed for disambiguation.
    #[serde(rename = "@referredType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referred_type: Option<String>,
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
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
impl std::ops::Deref for RelatedServiceOrderItem {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for RelatedServiceOrderItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
