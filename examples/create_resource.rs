//! Create Resource Example


use tmflib::tmf639::resource::Resource;
use tmflib::tmf629::customer::Customer;
use tmflib::tmf632::organization::Organization;
use tmflib::common::related_party::RelatedParty;

fn main() {

    // Create an organisation
    let organisation = Organization::new(String::from("An Organisation"));
    let customer = Customer::from(&organisation);
    let resource = Resource::new("VLAN")
        .add_party(RelatedParty::from(&customer));

    dbg!(resource);
}