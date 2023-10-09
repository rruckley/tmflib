//! Create Shipping Order Example
//! 

use tmflib::tmf700::shipping_order::ShippingOrder;

fn main() {
    let shipping_order = ShippingOrder::new();

    dbg!(shipping_order);
}