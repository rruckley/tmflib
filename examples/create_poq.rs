//! Create Product Offering Qualification Example
//! 
use tmflib::common::related_party::RelatedParty;
use tmflib::tmf620::product_offering::{ProductOffering,ProductOfferingRef};
use tmflib::tmf629::customer::Customer;
use tmflib::tmf679::product_qualification::ProductOfferingQualification;


fn main() {
    let customer = Customer::new(String::from("BigCustomer"));
    let offering = ProductOffering::new(String::from("MyOffer"));
    let mut poq = ProductOfferingQualification::new(Some(ProductOfferingRef::from(offering)));
    poq.add_party(RelatedParty::from(&customer));

    dbg!(poq);
}