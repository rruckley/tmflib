//! Create Catalog
//! 
use tmflib::common::related_party::RelatedParty;
use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::category::{Category,CategoryRef};
#[cfg(feature = "tmf632-v4")]
use tmflib::tmf632::organization_v4::Organization;
#[cfg(feature = "tmf632-v5")]
use tmflib::tmf632::organization_v5::Organization;
use tmflib::tmf629::customer::Customer;
use tmflib::HasRelatedParty;


fn main() {
    let org = Organization::new("A Customer");
    let cust = Customer::new(org);
    let mut catalog = Catalog::new("Customer A Catalog");
    let rel_party = RelatedParty::from(&cust);
    let cat = Category::new("Customer Category");
    catalog.add_party(rel_party);
    catalog.add_category(CategoryRef::from(&cat));

    dbg!(catalog);
}
