//! Create Permission Example

use tmflib::tmf672::permission::Permission;
use tmflib::tmf632::individual::Individual;
use tmflib::common::related_party::RelatedParty;

fn main() {
    let user = Individual::new("A User");
    let perm = Permission::new(RelatedParty::from(&user))
        .desc("A Description");

    dbg!(perm);
}