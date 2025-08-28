//! Shipping Order Item Module

use super::shipping_instruction::ShippingInstruction;
use crate::common::related_place::RelatedPlaceRefOrValue;
use crate::{HasId, Uri};
use serde::{Deserialize, Serialize};
use tmflib_derive::HasId;

use super::MOD_PATH;
const CLASS_PATH: &str = "shippingOrderItem";
const NEW_STATUS: &str = "New";

/// Shipping Item Action Type
#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
pub enum ShippingOrderItemActionType {
    /// Add new item
    #[default]
    Add,
    /// Modify item
    Modify,
    /// Delte item
    Delete,
    /// No change
    NoChange,
}

/// Shipping Order Item
#[derive(Clone, Default, Debug, Deserialize, HasId, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShippingOrderItem {
    /// Shipping Item Action Type
    pub action: ShippingOrderItemActionType,
    /// Uri for Shipping Order
    pub href: Option<Uri>,
    /// Unique Id for shipping item
    pub id: Option<String>,
    /// Quantity for this item
    pub quantity: String,
    /// Status of this order item
    pub status: String,
    // Referenced Types
    /// Shipping Destination
    pub place_to: Option<RelatedPlaceRefOrValue>,
    /// Shipping Instructions
    pub shipping_instruction: Option<ShippingInstruction>,
}

impl ShippingOrderItem {
    /// Create a new shipping order item
    pub fn new() -> ShippingOrderItem {
        ShippingOrderItem::create().status(NEW_STATUS)
    }

    fn status(mut self, status: impl Into<String>) -> ShippingOrderItem {
        self.status = status.into();
        self
    }

    /// Set shipping instructions for this order item
    pub fn instruction(mut self, instruction: ShippingInstruction) -> ShippingOrderItem {
        self.shipping_instruction = Some(instruction);
        self
    }
}

#[cfg(test)]
mod test {

    use crate::tmf700::shipping_instruction::ShippingInstruction;

    use super::{ShippingOrderItem, ShippingOrderItemActionType, NEW_STATUS};

    const ITEMACTION_JSON: &str = "\"Add\"";
    const ORDERITEM_JSON: &str = "{
        \"action\" : \"Add\",
        \"quantity\" : \"1\",
        \"status\" : \"Status\"
    }";

    #[test]
    fn shipping_item_new() {
        let item = ShippingOrderItem::new();

        assert_eq!(item.status, NEW_STATUS.to_string());
    }

    #[test]
    fn test_itemaction_deserialize() {
        let itemaction: ShippingOrderItemActionType =
            serde_json::from_str(ITEMACTION_JSON).unwrap();

        assert_eq!(itemaction, ShippingOrderItemActionType::Add);
    }

    #[test]
    fn test_orderitem_deserialize() {
        let orderitem: ShippingOrderItem = serde_json::from_str(ORDERITEM_JSON).unwrap();

        assert_eq!(orderitem.action, ShippingOrderItemActionType::Add);
        assert_eq!(orderitem.quantity.as_str(), "1");
        assert_eq!(orderitem.status.as_str(), "Status");
    }

    #[test]
    fn test_orderitem_instruction() {
        let orderitem =
            ShippingOrderItem::new().instruction(ShippingInstruction::new("SomeInstruction"));

        assert_eq!(orderitem.shipping_instruction.is_some(), true);
        assert_eq!(
            orderitem
                .shipping_instruction
                .unwrap()
                .label_message
                .is_some(),
            true
        );
    }
}
