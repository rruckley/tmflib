//! Create Product Offering Qualification Example
//! 
use tmflib::common::related_party::RelatedParty;
use tmflib::tmf620::product_offering::{ProductOffering,ProductOfferingRef};
use tmflib::tmf629::customer::Customer;
use tmflib::tmf632::organization::Organization;
use tmflib::tmf679::product_qualification::ProductOfferingQualification;


fn main() {
    let org = Organization::new(String::from("ACustomer"));
    let customer = Customer::new(org);
    let offering = ProductOffering::new(String::from("MyOffer"));
    let mut poq = ProductOfferingQualification::new(Some(ProductOfferingRef::from(offering)));
    poq.add_party(RelatedParty::from(&customer));

    dbg!(poq);
}