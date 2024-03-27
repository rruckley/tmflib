//! Hydrate Example

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
use tmflib::IsRef;

fn main() {
    // 1 Create object with related party

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

    // Now we wish to fake hydrating a related party
    let party = order.related_party.unwrap().iter().next().unwrap().clone();
    let full_party = party.hydrate(|h| {
        // Hydrate party via href
        // Issue here is related_party can point to multiple schema. 
        // How do we determine which one to hydrate to? via Role?
        // That implies we need role passed in.
        Some(h.clone())
    });

    println!("Found hdrate URL: {}",full_party.unwrap());
}