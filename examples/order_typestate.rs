//! Example of a typestate implementation for ProductOrder. This is a work in progress and not yet complete.

// use tmflib::tmf622::product_order_v4::{ProductOrder, ProductOrderStateType};
use tmflib::tmf622::product_order_typestate::{Order,Draft};

fn main() {
    let order = Order::<Draft>::new();

    let order = order.acknowledge().start().cancel("Customer requested cancellation");

    dbg!(order);
}