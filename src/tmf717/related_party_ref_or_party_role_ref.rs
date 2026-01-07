use serde::{Serialize, Deserialize};
use super::PartyRefOrPartyRoleRef;
use crate::common::extensible::Extensible;

/// Related Party Reference or Party Role Reference
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelatedPartyRefOrPartyRoleRef {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    /// Reference to a Party or Party Role playing the role of a Related Party in the context of the specific entity it is linked to.
    #[serde(rename = "partyOrPartyRole")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party_or_party_role: Option<PartyRefOrPartyRoleRef>,
    ///Role played by the related party or party role in the context of the specific entity it is linked to. Such as 'initiator', 'customer',  'salesAgent', 'user'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
impl std::fmt::Display for RelatedPartyRefOrPartyRoleRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for RelatedPartyRefOrPartyRoleRef {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for RelatedPartyRefOrPartyRoleRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
