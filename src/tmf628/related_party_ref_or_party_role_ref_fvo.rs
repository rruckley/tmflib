use super::{PartyRefOrPartyRoleRefFvo};
use serde::{Deserialize, Serialize};
use crate::common::extensible::ExtensibleFvo;

/// Related Party Ref Or Party Role Ref Fvo
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelatedPartyRefOrPartyRoleRefFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    ///
    #[serde(rename = "partyOrPartyRole")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party_or_party_role: Option<PartyRefOrPartyRoleRefFvo>,
    ///Role played by the related party or party role in the context of the specific entity it is linked to. Such as 'initiator', 'customer',  'salesAgent', 'user'
    pub role: String,
}
impl std::fmt::Display for RelatedPartyRefOrPartyRoleRefFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for RelatedPartyRefOrPartyRoleRefFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for RelatedPartyRefOrPartyRoleRefFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
