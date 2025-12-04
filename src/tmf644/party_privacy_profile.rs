use super::{
    PartyPrivacyAgreementRef, PartyPrivacyProfileCharacteristic,
    PartyPrivacyProfileSpecificationRef, RelatedPartyRefOrPartyRoleRef,
};
use crate::common::entity::Entity;
use crate::TimePeriod;
use serde::{Deserialize, Serialize};

/// Party Privacy Profile
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyPrivacyProfile {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Reference to the party (or party role) that agreed to the privacy profile
    #[serde(rename = "agreedByParty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agreed_by_party: Option<RelatedPartyRefOrPartyRoleRef>,
    ///Reference to Party Privacy Agreement resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agreement: Option<PartyPrivacyAgreementRef>,
    ///Reference to the party (or party role) to which the privacy profile applies
    #[serde(rename = "applicableForParty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applicable_for_party: Option<RelatedPartyRefOrPartyRoleRef>,
    ///The date on which the PartyPrivacyProfile was created
    #[serde(rename = "creationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<crate::DateTime>,
    ///Description of the privacy profile
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Date and time when the PartyPrivacyProfile was last updated
    #[serde(rename = "lastUpdate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update: Option<crate::DateTime>,
    ///Name of the privacy profile
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///List of characteristics of the privacy profile
    #[serde(rename = "partyPrivacyProfileCharacteristic")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub party_privacy_profile_characteristic: Vec<PartyPrivacyProfileCharacteristic>,
    ///Reference to Party Privacy Specification resource
    #[serde(rename = "partyPrivacyProfileSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party_privacy_profile_specification: Option<PartyPrivacyProfileSpecificationRef>,
    ///The status of this profile (for example: created, terminated, etc.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for PartyPrivacyProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PartyPrivacyProfile {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for PartyPrivacyProfile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
