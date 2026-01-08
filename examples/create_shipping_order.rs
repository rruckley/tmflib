//! Create Shipping Order Example
//!

#![allow(unused_imports)]

use tmflib::common::note::Note;
#[cfg(all(feature = "tmf700", feature = "build-V4"))]
use tmflib::tmf700::shipping_instruction::SignatureRequiredByType;
#[cfg(all(feature = "tmf700", feature = "build-V4"))]
use tmflib::tmf700::{
    shipping_instruction::ShippingInstruction, shipping_order::ShippingOrder,
    shipping_order_item::ShippingOrderItem,
};
use tmflib::HasNote;

fn main() {
    #[cfg(all(feature = "tmf700", feature = "build-V4"))]
    {
        let item = ShippingOrderItem::new().instruction(ShippingInstruction::new("An Instruction"));
        let mut shipping_order = ShippingOrder::new().instruction(
            ShippingInstruction::new("Top Level Instructions")
                .message("Please use the left dock")
                .signature_required_by(Some(SignatureRequiredByType::Receiver)),
        );
        shipping_order.add_item(item);
        shipping_order.add_note(Note::new("A Note"));

        let original_order = ShippingOrder::new();

        shipping_order.link_order(&original_order, "Original Order");
        dbg!(shipping_order);
    }
}
