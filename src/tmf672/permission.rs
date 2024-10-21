//! Permission Module

use crate::{LIB_PATH,HasId,DateTime,TimePeriod,Uri};
use tmflib_derive::HasId;
use serde::{Deserialize,Serialize};
use crate::common::related_party::RelatedParty;
use super::MOD_PATH;

const CLASS_PATH : &str = "permission";

/// User Permissions Struct
#[derive(Clone,Debug,Default,Deserialize,HasId,Serialize)]
pub struct Permission {
    creation_date: Option<DateTime>,
    description: Option<String>,
    href: Option<Uri>,
    id: Option<String>,
    valid_for: Option<TimePeriod>,
    // Referenced structures
    granter : Option<RelatedParty>,
    user: RelatedParty,
}

impl Permission {
    /// Create a new Permission
    pub fn new(party : RelatedParty) -> Permission {
        Permission::create().user(party)
    }

    /// Set the description of this permission
    pub fn desc(mut self, description : impl Into<String>) -> Permission {
        self.description = Some(description.into());
        self
    }

    /// Set the user for this permission
    pub fn user(mut self, party : RelatedParty) -> Permission {
        self.user = party;
        self
    }

    /// Set the graner for this permission
    pub fn granter(mut self, party : RelatedParty) -> Permission {
        self.granter = Some(party);
        self
    }
}

#[cfg(test)]
mod test {
    #[cfg(all(feature = "tmf632", feature = "build-V4"))]
    use crate::tmf632::individual_v4::Individual;
    #[cfg(all(feature = "tmf632", feature = "build-V5"))]
    use crate::tmf632::individual_v5::Individual;
    use crate::common::related_party::RelatedParty;

    use super::Permission;

    const IND : &str = "An Individual";
    const ADMIN: &str = "An Administrator";
    const DESC: &str = "A Description";

    const PERM_JSON : &str = "{
        \"description\" : \"A description\",
        \"user\" : { 
            \"id\" : \"U123\",
            \"href\" : \"http://example.com/user/U123\",
            \"name\" : \"John Q. Citizen\"
        }
    }";
    #[test]
    fn test_permission_new() {
        let user = Individual::new(IND);
        let perm = Permission::new(RelatedParty::from(&user));

        assert_eq!(perm.user,RelatedParty::from(&user))
    }

    #[test]
    fn test_permission_desc() {
        let user = Individual::new(IND);
        let perm = Permission::new(RelatedParty::from(&user))
            .desc(DESC);

        assert_eq!(perm.description, Some(DESC.into()));
    }

    #[test]
    fn test_permission_granter() {
        let user = Individual::new(IND);
        let admin = Individual::new(ADMIN);
        let party = RelatedParty::from(&user);
        let perm = Permission::new(party)
            .granter(RelatedParty::from(&admin));

        assert_eq!(perm.granter.is_some(),true);
        // assert_eq!(perm.granter.unwrap().name.is_some(),true);
        assert_eq!(perm.granter.unwrap().name.unwrap().as_str(),ADMIN);
    }

    #[test]
    fn test_permission_deserialize() {
        let perm : Permission = serde_json::from_str(PERM_JSON).unwrap();

        assert_eq!(perm.user.name.is_some(),true);
        assert_eq!(perm.user.name.unwrap().as_str(),"John Q. Citizen");
    }
}