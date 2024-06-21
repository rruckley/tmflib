//! Create Shipping Order Example
//! 

use tmflib::tmf700::shipping_instruction::SignatureRequiredByType;
use tmflib::tmf700::{shipping_order::ShippingOrder,shipping_order_item::ShippingOrderItem,shipping_instruction::ShippingInstruction};
use tmflib::common::note::Note;
use tmflib::HasNote;

fn main() {
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

    dbg!(shipping_order);
}