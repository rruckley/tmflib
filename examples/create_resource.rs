//! Create Resource Example

#![allow(unused_imports)]

#[cfg(all(feature = "tmf639", feature = "build-V4"))]
use tmflib::tmf639::resource::Resource;
use tmflib::tmf629::customer::Customer;
#[cfg(all(feature = "tmf632", feature = "build-V4"))]
use tmflib::tmf632::organization_v4::Organization;
#[cfg(all(feature = "tmf632", feature = "build-V5"))]
use tmflib::tmf632::organization_v5::Organization;
use tmflib::common::related_party::RelatedParty;
use tmflib::HasRelatedParty;

fn main() {
    #[cfg(all(feature = "tmf639", feature = "build-V4"))]
    {
    // Create an organisation
    let organisation = Organization::new("An Organisation");
    let customer = Customer::from(&organisation);
    let mut resource = Resource::new("VLAN");
    resource.add_party(RelatedParty::from(&customer));

    dbg!(resource);
    }
}