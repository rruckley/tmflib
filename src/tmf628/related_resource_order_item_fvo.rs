use super::OrderItemActionType;
use crate::common::extensible::ExtensibleFvo;
use serde::{Deserialize, Serialize};

///Related Resource Order Item FVO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedResourceOrderItemFvo {
    ///The actual type of the target instance when needed for disambiguation.
    #[serde(rename = "@referredType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referred_type: Option<String>,
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    ///action to be performed on the product
    #[serde(rename = "itemAction")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_action: Option<OrderItemActionType>,
    ///Identifier of the order item where the resource was managed
    #[serde(rename = "itemId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    ///Reference of the related entity.
    #[serde(rename = "resourceOrderHref")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_order_href: Option<String>,
    ///Unique identifier of a related entity.
    #[serde(rename = "resourceOrderId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_order_id: Option<String>,
    ///role of the resource order item for this resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
impl std::fmt::Display for RelatedResourceOrderItemFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for RelatedResourceOrderItemFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for RelatedResourceOrderItemFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
