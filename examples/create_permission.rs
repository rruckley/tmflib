//! Create Permission Example

use tmflib::tmf672::permission::Permission;
#[cfg(feature = "tmf632-v4")]
use tmflib::tmf632::individual_v4::Individual;
#[cfg(feature = "tmf632-v5")]
use tmflib::tmf632::individual_v5::Individual;
use tmflib::common::related_party::RelatedParty;

fn main() {
    let user = Individual::new("A User");
    let perm = Permission::new(RelatedParty::from(&user))
        .desc("A Description");

    dbg!(perm);
}