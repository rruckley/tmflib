//! Create Product Offering Qualification Example
//! 
use tmflib::common::related_party::RelatedParty;
#[cfg(feature = "tmf620-v4")]
use tmflib::tmf620::product_offering::{ProductOffering,ProductOfferingRef};
#[cfg(feature = "tmf620-v5")]
use tmflib::tmf620::product_offering_v5::{ProductOffering,ProductOfferingRef};
use tmflib::tmf629::customer::Customer;
#[cfg(feature = "tmf632-v4")]
use tmflib::tmf632::organization_v4::Organization;
use tmflib::tmf679::product_qualification::ProductOfferingQualification;


fn main() {
    let org = Organization::new("ACustomer");
    let customer = Customer::new(org);
    let offering = ProductOffering::new("MyOffer");
    let mut poq = ProductOfferingQualification::new(Some(ProductOfferingRef::from(offering)));
    poq.add_party(RelatedParty::from(&customer));

    dbg!(poq);
}