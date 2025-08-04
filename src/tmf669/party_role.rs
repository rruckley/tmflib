//! Party Role Module

use serde::{Deserialize, Serialize};

use crate::common::tmf_error::TMFError;
use crate::{
    common::{contact::ContactMedium, related_party::RelatedParty},
    tmf632::Characteristic,
    tmf651::agreement::AgreementRef,
    tmf666::{AccountRef, PaymentMethodRef},
    DateTime, HasId, HasName, HasRelatedParty, HasValidity, TimePeriod, Uri, LIB_PATH,
};
use tmflib_derive::{HasId, HasName, HasRelatedParty, HasValidity};

use super::MOD_PATH;

const CLASS_PATH: &str = "partyRole";

/// Party Role - Credit Profile
#[derive(Clone, Debug, Default, Deserialize, HasValidity, Serialize)]
pub struct CreditProfile {
    /// Credit Profile Date
    pub credit_profile_date: Option<DateTime>,
    /// Credit Risk Rating
    pub credit_risk_rating: Option<u32>,
    /// Credit Score
    pub credit_score: Option<u32>,
    /// Profile Validity
    pub valid_for: Option<TimePeriod>,
}

/// Party Role
#[derive(
    Clone, Debug, Default, Deserialize, HasId, HasName, HasRelatedParty, HasValidity, Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct PartyRole {
    /// Id of the Party Role
    pub id: Option<String>,
    /// HTML link
    pub href: Option<Uri>,
    /// Name of Role
    pub name: Option<String>,
    /// Role Status
    pub status: Option<String>,
    /// Reason for current status
    pub status_reason: Option<String>,
    /// Validity Period
    pub valid_for: Option<TimePeriod>,
    /// Entity that is associated with this role
    pub engaged_party: Option<RelatedParty>,
    /// Other related parties
    pub related_party: Option<Vec<RelatedParty>>,
    /// Credit Profiles
    pub credit_profile: Option<Vec<CreditProfile>>,
    /// Agreements
    pub agreement: Option<Vec<AgreementRef>>,
    /// Accounts
    pub account: Option<Vec<AccountRef>>,
    /// Payment Methods
    pub payment_method: Option<Vec<PaymentMethodRef>>,
    /// Contact Media
    pub contact_medium: Option<Vec<ContactMedium>>,
    /// Party Role Characteristics
    pub characteristic: Option<Vec<Characteristic>>,
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
    pub fn new(name: impl Into<String>, party: RelatedParty) -> PartyRole {
        PartyRole {
            name: Some(name.into()),
            engaged_party: Some(party),
            ..PartyRole::create()
        }
    }

    /// Set engaged party (Using [RelatedParty] reference)
    pub fn engaged_party(mut self, related_party: RelatedParty) -> PartyRole {
        self.engaged_party = Some(related_party);
        self
    }

    /// Add a new credit profile
    pub fn add_profile(&mut self, profile: CreditProfile) {
        match self.credit_profile.as_mut() {
            Some(cp) => cp.push(profile),
            None => self.credit_profile = Some(vec![profile]),
        }
    }

    /// Get Profile by index
    pub fn get_profile(&self, idx: usize) -> Option<&CreditProfile> {
        match self.credit_profile.as_ref() {
            Some(cp) => cp.get(idx),
            None => None,
        }
    }

    /// Add new payment method to this Party Role
    pub fn add_payment(&mut self, payment: PaymentMethodRef) {
        match self.payment_method.as_mut() {
            Some(pm) => pm.push(payment),
            None => self.payment_method = Some(vec![payment]),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::related_party::RelatedParty;
    #[cfg(all(feature = "tmf632", feature = "build-V4"))]
    use crate::tmf632::individual_v4::Individual;
    #[cfg(all(feature = "tmf632", feature = "build-V5"))]
    use crate::tmf632::individual_v5::Individual;

    use super::*;

    const ROLE_NAME: &str = "APartyRole";
    const IND1: &str = "Individual1";
    const IND2: &str = "Individual2";

    const PARTYROLE_JSON: &str = "{
        \"name\" : \"PartyRoleName\"
    }";

    #[test]
    fn test_party_role_new_name() {
        let ind = Individual::new(IND1);
        let role = PartyRole::new(ROLE_NAME, RelatedParty::from(&ind));

        assert_eq!(role.name.is_some(), true);
        assert_eq!(role.get_name().as_str(), ROLE_NAME);
        assert_eq!(role.engaged_party.is_some(), true);
        assert_eq!(
            role.engaged_party.as_ref().unwrap().name,
            Some(ind.get_name())
        );
    }

    #[test]
    fn test_partyrole_deserialize() {
        let partyrole: PartyRole = serde_json::from_str(PARTYROLE_JSON).unwrap();

        assert_eq!(partyrole.get_name().as_str(), "PartyRoleName");
    }

    #[test]
    fn test_partyrole_engagedparty() {
        let ind1 = Individual::new(IND1);
        let ind2 = Individual::new(IND2);
        let party2 = RelatedParty::from(&ind2);
        let party1 = RelatedParty::from(&ind1);
        let partyrole = PartyRole::new(ROLE_NAME, party1).engaged_party(party2);

        assert_eq!(partyrole.engaged_party.is_some(), true);
        assert_eq!(partyrole.engaged_party.unwrap().name.is_some(), true);
    }
}
