//! Create Permission Example

#![allow(unused_imports)]

use tmflib::common::related_party::RelatedParty;
#[cfg(all(feature = "tmf632", feature = "build-V4"))]
use tmflib::tmf632::individual_v4::Individual;
#[cfg(all(feature = "tmf632", feature = "build-V5"))]
use tmflib::tmf632::individual_v5::Individual;
#[cfg(all(feature = "tmf672", feature = "build-V4"))]
use tmflib::tmf672::permission::Permission;

fn main() {
    #[cfg(all(feature = "tmf672", feature = "build-V4"))]
    {
        let user = Individual::new("A User");
        let perm = Permission::new(RelatedParty::from(&user)).desc("A Description");

        dbg!(perm);
    }
}
