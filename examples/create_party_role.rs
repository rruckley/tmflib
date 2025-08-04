//! Create Party Role Example
//! # Description
//! This example shows creating a party role "Account Manager", connecting
//! it to an individual and associating it with a customer.
//! This role could then be linked into an order or quote as a related party.

use tmflib::common::related_party::RelatedParty;
use tmflib::tmf629::customer::Customer;
#[cfg(all(feature = "tmf632", feature = "build-V4"))]
use tmflib::tmf632::individual_v4::Individual;
#[cfg(all(feature = "tmf632", feature = "build-V5"))]
use tmflib::tmf632::individual_v5::Individual;
#[cfg(all(feature = "tmf632", feature = "build-V4"))]
use tmflib::tmf632::organization_v4::Organization;
#[cfg(all(feature = "tmf632", feature = "build-V5"))]
use tmflib::tmf632::organization_v5::Organization;
use tmflib::tmf669::party_role::PartyRole;
use tmflib::HasRelatedParty;

fn main() {
    let individual = Individual::new("John Smith");
    let organisation = Organization::new("A Customer".to_string());
    let customer = Customer::new(organisation);
    let mut role = PartyRole::new("Account Manager", RelatedParty::from(&individual));
    role.add_party(RelatedParty::from(&customer));
    dbg!(role);
}
