//! Shipping Order Module
//! 


use super::{shipping_instruction::ShippingInstruction, shipping_order_item::ShippingOrderItem};
use crate::{common::note::Note, DateTime};
#[cfg(feature = "tmf622-v4")]
use crate::tmf622::product_order_v4::ProductOrderRef;
#[cfg(feature = "tmf622-v5")]
use crate::tmf622::product_order_v5::ProductOrderRef;

use super::MOD_PATH;
use crate::{
    HasId,
    HasNote,
    LIB_PATH
};
use tmflib_derive::{
    HasId,
    HasNote
};

use serde::{Deserialize,Serialize};

const CLASS_PATH : &str = "shippingOrder";

/// Related Shipping Order
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct RelatedShippingOrder {
    href: String,
    id  : String,
    name: String,
    role: Option<String>,
}

impl RelatedShippingOrder {
    /// Set the role for this RelatedShippingOrder
    pub fn role(mut self, role : impl Into<String>) -> RelatedShippingOrder {
        self.role = Some(role.into());
        self
    }
}

impl From<&ShippingOrder> for RelatedShippingOrder {
    fn from(value: &ShippingOrder) -> Self {
        // Generate Ref from SO
        RelatedShippingOrder {
            href: value.get_href(),
            id: value.get_id(),
            name: String::default(),
            role: None,
        }
    }
}

/// Order for shipping of tangible goods
#[derive(Clone, Debug, Default, Deserialize, HasId, HasNote, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShippingOrder {
    /// Creation Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<DateTime>,
    /// Last Update Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_date: Option<DateTime>,
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTML Reference to this object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status : Option<String>,
    //
    // Referenced Types
    //
    /// Shipping Line Items
    pub shipping_order_item: Vec<ShippingOrderItem>,
    /// Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Note>>,
    /// Related Shipping Order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_shipping_order: Option<RelatedShippingOrder>,
    /// Product Order Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_order: Option<ProductOrderRef>,
    /// Shipping Instruction Top Level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_instruction : Option<ShippingInstruction>,
}

impl ShippingOrder {
    /// Create new ShippingOrder
    pub fn new() -> ShippingOrder {
        let mut order = ShippingOrder::create();
        order.note = Some(vec![]);
        order
    }

    /// Set shipping instructions for this shipping order
    pub fn instruction(mut self, instruction : ShippingInstruction) -> ShippingOrder {
        self.shipping_instruction = Some(instruction);
        self
    }

    /// Add an order item to this order
    pub fn add_item(&mut self, item : ShippingOrderItem) {
        self.shipping_order_item.push(item);
    }

    /// Add a RelatedShippingOrder to this order
    pub fn link_order(&mut self, shipping_order: &ShippingOrder, role : impl Into<String>) {
        self.related_shipping_order = Some(RelatedShippingOrder::from(shipping_order).role(role));  
    }
}
#[cfg(test)]
mod test {
    use super::RelatedShippingOrder;
    use super::ShippingOrder;
    use super::HasId;
    #[test]
    fn shipping_order_create_id() {
        // Generate shipping order, test id
        let so = ShippingOrder::new();

        assert_eq!(so.id.is_some(),true);
    }

    #[test]
    fn shipping_order_create_href() {
        let so = ShippingOrder::new();

        assert_eq!(so.href.is_some(), true);
    }

    #[test]
    fn shipping_order_create_href_matches_id() {
        let so = ShippingOrder::new();
        let id = so.get_id();
        let href = so.get_href();

        assert!(href.contains(&id));
    }

    #[test]
    fn shipping_order_related() {
        let so = ShippingOrder::new();
        let so_rel = RelatedShippingOrder::from(&so);

        assert_eq!(so.get_id(),so_rel.id);
        assert_eq!(so.get_href(),so_rel.href);
    }

    #[test]
    fn shiping_order_add_related() {
        let so_parent = ShippingOrder::new();
        let mut so_child = ShippingOrder::new();

        so_child.link_order(&so_parent, "Parent");
        
        assert_eq!(so_child.related_shipping_order.is_some(),true);
        let linked_order = so_child.related_shipping_order.unwrap();
        
        assert_eq!(so_parent.get_id(),linked_order.id);
    }
}