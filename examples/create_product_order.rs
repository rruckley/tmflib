//! Create Product Order Example
use tmflib::common::related_party::RelatedParty;
use tmflib::tmf622::product_order::ProductOrder;
use tmflib::tmf622::product_order_item::ProductOrderItem;
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf629::customer::Customer;
use tmflib::tmf632::individual::Individual;

fn main() {
    // This example simple creates in memory structures without reference to any persistence
    let offer = ProductOffering::new(String::from("Sample Offering"));
    let customer = Customer::new(String::from("Sample Customer"));
    let person = Individual::new("Ryan Ruckley".to_string());
    let mut order = ProductOrder::new();
    order.add_order_item(ProductOrderItem::from(offer));
    order.add_party(RelatedParty::from(&customer));
    order.add_party(RelatedParty::from(&person));
    dbg!(order);
}