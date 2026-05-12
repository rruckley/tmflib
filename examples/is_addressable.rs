//! Show example using isAddreessable Trait
//!

use tmflib::tmf632::individual_v4::Individual;
use tmflib::IsAddressable;

fn main() {
    let objects = Individual::get_objects();
    dbg!(objects);
}
