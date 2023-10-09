//! Create Product Order Example
use tmflib::common::related_party::RelatedParty;
use tmflib::tmf622::product_order::ProductOrder;
use tmflib::tmf622::product_order_item::ProductOrderItem;
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf629::customer::Customer;

fn main() {
    let offer = ProductOffering::new(String::from("Sample Offering"));
    let customer = Customer::new(String::from("Sample Customer"));
    let mut order = ProductOrder::new();
    order.add_order_item(ProductOrderItem::from(offer));
    order.add_party(RelatedParty::from(&customer));

    dbg!(order);
}