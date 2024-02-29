//! Shipping Order Module
//! 


use super::shipping_instruction::ShippingInstruction;
use super::shipping_order_item::ShippingOrderItem;
use crate::common::note::Note;
use super::MOD_PATH;
use super::{HasId,CreateTMF,LIB_PATH};
use tmflib_derive::HasId;

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

impl From<ShippingOrder> for RelatedShippingOrder {
    fn from(value: ShippingOrder) -> Self {
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
#[derive(Clone, Debug, Default, Deserialize, HasId, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShippingOrder {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTML Reference to this object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    // Referenced Types
    /// Shipping Line Items
    pub shipping_order_item: Vec<ShippingOrderItem>,
    /// Notes
    pub note: Vec<Note>,
    /// Related Shipping Order
    pub related_shipping_order: Option<RelatedShippingOrder>,
}

impl ShippingOrder {
    /// Create new ShippingOrder
    pub fn new() -> ShippingOrder {
        ShippingOrder::create()
    }

    /// Add an order item to this order
    pub fn add_item(&mut self, item : ShippingOrderItem) {
        self.shipping_order_item.push(item);
    }

    /// Add Note
    pub fn add_note(&mut self, note : Note) {
        self.note.push(note);
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
        let so_rel = RelatedShippingOrder::from(so);

        assert_eq!(so.get_id(),so_rel.id);
        assert_eq!(so.get_href(),so_rel.href);
    }
}