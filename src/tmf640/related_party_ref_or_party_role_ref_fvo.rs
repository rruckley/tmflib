use serde::{Serialize, Deserialize};
use super::{Extensible, PartyRefOrPartyRoleRefFvo};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelatedPartyRefOrPartyRoleRefFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
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
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for RelatedPartyRefOrPartyRoleRefFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
