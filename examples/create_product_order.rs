//! Create Product Order Example

#![allow(unused_imports)]

use tmflib::common::contact::ContactMedium;
use tmflib::common::related_party::RelatedParty;
#[cfg(all(feature = "tmf622", feature = "build-V4"))]
use tmflib::tmf622::product_order_v4::ProductOrder;
#[cfg(all(feature = "tmf622", feature = "build-V5"))]
use tmflib::tmf622::product_order_v5::ProductOrder;
#[cfg(all(feature = "tmf622", feature = "build-V4"))]
use tmflib::tmf622::product_order_item::ProductOrderItem;
#[cfg(all(feature = "tmf620", feature = "build-V4"))]
use tmflib::tmf620::product_offering::ProductOffering;
#[cfg(all(feature = "tmf620", feature = "build-V5"))]
use tmflib::tmf620::product_offering_v5::ProductOffering;
use tmflib::tmf629::customer::Customer;
#[cfg(all(feature = "tmf632", feature = "build-V4"))]
use tmflib::tmf632::individual_v4::Individual;
#[cfg(all(feature = "tmf632", feature = "build-V4"))]
use tmflib::tmf632::organization_v4::Organization;
#[cfg(all(feature = "tmf632", feature = "build-V5"))]
use tmflib::tmf632::individual_v5::Individual;
#[cfg(all(feature = "tmf632", feature = "build-V5"))]
use tmflib::tmf632::organization_v5::Organization;
use tmflib::HasRelatedParty;

fn main() {
    #[cfg(all(feature = "tmf622", feature = "build-V4"))]
    {
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
}