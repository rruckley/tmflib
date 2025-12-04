use super::{CharacteristicMvo, Extensible, RelatedPartyRefOrPartyRoleRefMvo};
use serde::{Deserialize, Serialize};

/// Party Privacy Profile Characteristic MVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyPrivacyProfileCharacteristicMvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///The characteristic to which the privacy rules apply
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub characterisitc: Option<CharacteristicMvo>,
    ///Defines the purpose authorized or refused for the characteristic (e.g. ADMIN, INFORMATION, MARKETING, RESEARCH, etc.
    #[serde(rename = "privacyUsagePurpose")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_usage_purpose: Option<String>,
    ///A list of parties to which the allowed use of the characteristic applies.
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_party: Vec<RelatedPartyRefOrPartyRoleRefMvo>,
}
impl std::fmt::Display for PartyPrivacyProfileCharacteristicMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PartyPrivacyProfileCharacteristicMvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for PartyPrivacyProfileCharacteristicMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
