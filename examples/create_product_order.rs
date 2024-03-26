//! Create Product Order Example
use tmflib::common::contact::ContactMedium;
use tmflib::common::related_party::RelatedParty;
#[cfg(feature = "tmf622-v4")]
use tmflib::tmf622::product_order_v4::ProductOrder;
#[cfg(feature = "tmf622-v5")]
use tmflib::tmf622::product_order_v5::ProductOrder;
use tmflib::tmf622::product_order_item::ProductOrderItem;
#[cfg(feature = "tmf620-v4")]
use tmflib::tmf620::product_offering::ProductOffering;
#[cfg(feature = "tmf620-v5")]
use tmflib::tmf620::product_offering_v5::ProductOffering;
use tmflib::tmf629::customer::Customer;
use tmflib::tmf632::individual::Individual;
use tmflib::tmf632::organization::Organization;

fn main() {
    // This example simple creates in memory structures without reference to any persistence
    let offer = ProductOffering::new("Sample Offering");
    let org = Organization::new("ACustomer");
    let customer = Customer::new(org);
    let mut person = Individual::new("John Smith");
    person.add_contact(ContactMedium::email("John.Smith@example.com"));
    let mut order = ProductOrder::new();
    order.add_order_item(ProductOrderItem::from(offer));
    order.add_party(RelatedParty::from(&customer));
    order.add_party(RelatedParty::from(&person));
    dbg!(order);
}