//! Shipping Order Item Module

use crate::{LIB_PATH,HasId,CreateTMF,Uri};
use crate::common::related_place::RelatedPlaceRefOrValue;
use super::shipping_instruction::ShippingInstruction;
use tmflib_derive::HasId;
use serde::{Deserialize,Serialize};

use super::MOD_PATH;
const CLASS_PATH : &str = "shippingOrderItem";

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