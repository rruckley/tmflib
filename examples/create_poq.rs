//! Create Product Offering Qualification Example
//! 
#![allow(unused_imports)]

use tmflib::common::related_party::RelatedParty;
#[cfg(all(feature = "tmf620", feature = "build-V4"))]
use tmflib::tmf620::product_offering::{ProductOffering,ProductOfferingRef};
#[cfg(all(feature = "tmf620", feature = "build-V5"))]
use tmflib::tmf620::product_offering_v5::{ProductOffering,ProductOfferingRef};
use tmflib::tmf629::customer::Customer;
#[cfg(all(feature = "tmf632", feature = "build-V4"))]
use tmflib::tmf632::organization_v4::Organization;
#[cfg(all(feature = "tmf632", feature = "build-V5"))]
use tmflib::tmf632::organization_v5::Organization;
#[cfg(all(feature = "tmf679", feature = "build-V4"))]
use tmflib::tmf679::product_qualification::ProductOfferingQualification;
use tmflib::HasRelatedParty;


fn main() {
    #[cfg(all(feature = "tmf679", feature = "build-V4"))]
    {
        let org = Organization::new("ACustomer");
        let customer = Customer::new(org);
        let offering = ProductOffering::new("MyOffer");
        let mut poq = ProductOfferingQualification::new(Some(ProductOfferingRef::from(offering)));
        poq.add_party(RelatedParty::from(&customer));
    
        dbg!(poq);    
    }
}