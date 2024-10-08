//! Party Role Module

use serde::{Deserialize, Serialize};

use crate::{HasId,HasName, HasRelatedParty, LIB_PATH, common::related_party::RelatedParty};
use tmflib_derive::{HasId,HasName,HasRelatedParty};

use super::MOD_PATH;

const CLASS_PATH : &str = "partyRole";

/// Party Role
#[derive(Clone, Debug, Default, Deserialize, HasId, HasName, HasRelatedParty, Serialize)]
pub struct PartyRole {
    /// Id of the Party Role
   pub id: Option<String>,
   /// HTML link
   pub href: Option<String>,
   /// Name of Role
   pub name: Option<String>,
   /// Entity that is associated with this role
   engaged_party: Option<RelatedParty>,
   /// Other related parties
   related_party: Option<Vec<RelatedParty>>,
}

impl PartyRole {
    /// Create new PartyRole based on a given [crate::tmf632::individual_v4::Individual].
    /// ```
    /// # use tmflib::tmf669::party_role::PartyRole;
    /// use tmflib::common::related_party::RelatedParty;
    /// #[cfg(all(feature = "tmf632", feature = "build-V4"))]
    /// use tmflib::tmf632::individual_v4::Individual;
    /// #[cfg(all(feature = "tmf632", feature = "build-V5"))]
    /// use tmflib::tmf632::individual_v5::Individual;
    /// let individual = Individual::new("John Smith");
    /// let role = PartyRole::new("Account Manager",RelatedParty::from(&individual));
    /// ```
    pub fn new(name : impl Into<String>,party : RelatedParty) -> PartyRole {
        PartyRole {
            name : Some(name.into()),
            engaged_party: Some(party),
            ..PartyRole::create()
        }
    }

    /// Set engaged party (Using [RelatedParty] reference)
    pub fn engaged_party(mut self, related_party: RelatedParty) -> PartyRole {
        self.engaged_party = Some(related_party);
        self
    }
}

#[cfg(test)]
mod test {
    use crate::tmf632::individual_v4::Individual;
    use crate::common::related_party::RelatedParty;

    use super::*;

    const ROLE_NAME : &str = "APartyRole";
    const IND1 : &str = "Individual1";
    const IND2 : &str = "Individual2";

    const PARTYROLE_JSON : &str = "{
        \"name\" : \"PartyRoleName\"
    }";  

    #[test]
    fn test_party_role_new_name() {
        let ind = Individual::new(IND1);
        let role = PartyRole::new(ROLE_NAME,RelatedParty::from(&ind));

        assert_eq!(role.name.is_some(),true);
        assert_eq!(role.get_name().as_str(),ROLE_NAME);
        assert_eq!(role.engaged_party.is_some(),true);
        assert_eq!(role.engaged_party.as_ref().unwrap().name,Some(ind.get_name()));
    }

    #[test]
    fn test_partyrole_deserialize() {
        let partyrole : PartyRole = serde_json::from_str(PARTYROLE_JSON).unwrap();

        assert_eq!(partyrole.get_name().as_str(),"PartyRoleName");
    }

    #[test]
    fn test_partyrole_engagedparty() {
        let ind1 = Individual::new(IND1);
        let ind2 = Individual::new(IND2);
        let party2 = RelatedParty::from(&ind2);
        let party1 = RelatedParty::from(&ind1);
        let partyrole = PartyRole::new(ROLE_NAME, party1)
            .engaged_party(party2);

        assert_eq!(partyrole.engaged_party.is_some(),true);
        assert_eq!(partyrole.engaged_party.unwrap().name.is_some(),true);
    }
}