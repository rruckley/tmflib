//! Create Product Order Example
use tmflib::common::contact::ContactMedium;
use tmflib::common::related_party::RelatedParty;
use tmflib::tmf622::product_order::ProductOrder;
use tmflib::tmf622::product_order_item::ProductOrderItem;
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf629::customer::Customer;
use tmflib::tmf632::individual::Individual;
use tmflib::tmf632::organization::Organization;

fn main() {
    // This example simple creates in memory structures without reference to any persistence
    let offer = ProductOffering::new(String::from("Sample Offering"));
    let org = Organization::new(String::from("ACustomer"));
    let customer = Customer::new(org);
    let mut person = Individual::new("John Smith");
    person.add_contact(ContactMedium::email("John.Smith@example.com"));
    let mut order = ProductOrder::new();
    order.add_order_item(ProductOrderItem::from(offer));
    order.add_party(RelatedParty::from(&customer));
    order.add_party(RelatedParty::from(&person));
    dbg!(order);
}