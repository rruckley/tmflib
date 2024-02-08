//! Create Party Role Example
//! # Description
//! This example shows creating a party role "Account Manager", connecting
//! it to an individual and associating it with a customer.
//! This role could then be linked into an order or quote as a related party.


use tmflib::tmf669::party_role::PartyRole;
use tmflib::tmf629::customer::Customer;
use tmflib::tmf632::individual::Individual;
use tmflib::tmf632::organization::Organization;
use tmflib::common::related_party::RelatedParty;

fn main() {
    let individual = Individual::new("John Smith");
    let organisation = Organization::new("A Customer".to_string());
    let customer = Customer::new(organisation);
    let mut role = PartyRole::new("Account Manager",RelatedParty::from(&individual));
    role.add_party(RelatedParty::from(&customer));
    dbg!(role);
}