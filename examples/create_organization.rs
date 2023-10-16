//! Create Organization Example

use tmflib::tmf632::organization::Organization;

fn main() {

    // Two methods to create an org object

    // 1) use native constructor new()
    let org1 = Organization::new(String::from("My Organization"));
    // 2) Use conversion from a String.
    let org2 : Organization = String::from("My Organisation").into();

    dbg!(org1);
    dbg!(org2);
}