//! Shipping Order Item Module

use crate::{LIB_PATH,HasId,Uri};
use crate::common::related_place::RelatedPlaceRefOrValue;
use super::shipping_instruction::ShippingInstruction;
use tmflib_derive::HasId;
use serde::{Deserialize,Serialize};

use super::MOD_PATH;
const CLASS_PATH : &str = "shippingOrderItem";
const NEW_STATUS : &str = "New";

/// Shipping Item Action Type
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
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
#[derive(Clone,Default,Debug,Deserialize,HasId,Serialize)]
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
        ShippingOrderItem::create()
            .status(NEW_STATUS)
    }
    
    fn status(mut self, status : impl Into<String>) -> ShippingOrderItem {
        self.status = status.into();
        self
    }

    /// Set shipping instructions for this order item
    pub fn instruction (mut self, instruction : ShippingInstruction) -> ShippingOrderItem {
        self.shipping_instruction = Some(instruction);
        self
    }
}

#[cfg(test)]
mod test {
    use super::{ShippingOrderItem, NEW_STATUS};

    #[test]
    fn shipping_item_new() {
        let item = ShippingOrderItem::new();

        assert_eq!(item.status, NEW_STATUS.to_string());
    }
}