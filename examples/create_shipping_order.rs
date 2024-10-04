//! Create Shipping Order Example
//! 

#![allow(unused_imports)]

#[cfg(feature="tmf700-v4")]
use tmflib::tmf700::shipping_instruction::SignatureRequiredByType;
#[cfg(feature="tmf700-v4")]
use tmflib::tmf700::{shipping_order::ShippingOrder,shipping_order_item::ShippingOrderItem,shipping_instruction::ShippingInstruction};
use tmflib::common::note::Note;
use tmflib::HasNote;

fn main() {
    #[cfg(feature = "tmf700-v4")]
    {
        let item = ShippingOrderItem::new()
        .instruction(ShippingInstruction::new("An Instruction"));
    let mut shipping_order = ShippingOrder::new()
        .instruction(
            ShippingInstruction::new("Top Level Instructions")
            .message("Please use the left dock")
            .signature_required_by(Some(SignatureRequiredByType::Receiver))
    );
    shipping_order.add_item(item);
    shipping_order.add_note(Note::new("A Note"));

    let original_order = ShippingOrder::new();
    
    shipping_order.link_order(&original_order, "Original Order");
    dbg!(shipping_order);

    }
}