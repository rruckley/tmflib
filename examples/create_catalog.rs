//! Create Catalog
//! 
use tmflib::common::related_party::RelatedParty;
use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::category::{Category,CategoryRef};
use tmflib::tmf632::organization::Organization;
use tmflib::tmf629::customer::Customer;


fn main() {
    let org = Organization::new("A Customer".to_string());
    let cust = Customer::new(org);
    let mut catalog = Catalog::new("Customer A Catalog");
    let rel_party = RelatedParty::from(&cust);
    let cat = Category::new("Customer Category".to_string());
    catalog.add_party(rel_party);
    catalog.add_category(CategoryRef::from(&cat));

    dbg!(catalog);
}
