use serde::{Serialize, Deserialize};
use super::{ContactMedium, PartyRef, RelatedPartyOrPartyRole};
use crate::common::entity::Entity;
use crate::TimePeriod;

/// Customer360 Customer VO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Customer360CustomerVo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///A list of contact details (phone, email, etc.) for the Customer
    #[serde(rename = "contactMedium")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact_medium: Vec<ContactMedium>,
    ///A Party reference
    #[serde(rename = "engagedParty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engaged_party: Option<PartyRef>,
    ///A word, term, or phrase by which the Customer is known and distinguished from other Customers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///A list of parties or party roles related to the Customer
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_party: Vec<RelatedPartyOrPartyRole>,
    ///Used to track the lifecycle status of the party role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for Customer360CustomerVo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Customer360CustomerVo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Customer360CustomerVo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
