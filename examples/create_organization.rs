//! Create Organization Example

#[cfg(feature = "tmf632-v4")]
use tmflib::tmf632::organization_v4::Organization;

fn main() {

    // Two methods to create an org object

    // 1) use native constructor new()
    let org1 = Organization::new("My Organization");
    // 2) Use conversion from a String.
    let org2 : Organization = String::from("Organisation from String").into();

    dbg!(org1);
    dbg!(org2);
}