//! Create Product Offering Qualification Example
//! 
use tmflib::common::related_party::RelatedParty;
use tmflib::tmf679::product_qualification::ProductOfferingQualification;
use tmflib::tmf629::customer::Customer;

fn main() {
    let customer = Customer::new(String::from("BigCustomer"));
    let mut poq = ProductOfferingQualification::new();
    poq.add_party(RelatedParty::from(&customer));

    dbg!(poq);
}