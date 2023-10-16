//! Create Organization Example

use tmflib::tmf632::organization::Organization;

fn main() {
    let organization = Organization::new(String::from("MyOrganization"));

    dbg!(organization);
}