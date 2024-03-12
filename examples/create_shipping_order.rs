//! Create Shipping Order Example
//! 

use tmflib::tmf700::{shipping_order::ShippingOrder,shipping_order_item::ShippingOrderItem,shipping_instruction::ShippingInstruction};
use tmflib::common::note::Note;

fn main() {
    let item = ShippingOrderItem::new()
        .instruction(ShippingInstruction::new("An Instruction"));
    let mut shipping_order = ShippingOrder::new();
    shipping_order.add_item(item);
    shipping_order.add_note(Note::new("A Note"));

    dbg!(shipping_order);
}