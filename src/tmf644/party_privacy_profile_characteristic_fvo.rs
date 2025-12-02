use serde::{Serialize, Deserialize};
use super::{CharacteristicFvo, ExtensibleFvo, RelatedPartyRefOrPartyRoleRefFvo};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyPrivacyProfileCharacteristicFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub characterisitc: Option<CharacteristicFvo>,
    ///Defines the purpose authorized or refused for the characteristic (e.g. ADMIN, INFORMATION, MARKETING, RESEARCH, etc.
    #[serde(rename = "privacyUsagePurpose")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_usage_purpose: Option<String>,
    ///A list of parties to which the allowed use of the characteristic applies.
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_party: Vec<RelatedPartyRefOrPartyRoleRefFvo>,
}
impl std::fmt::Display for PartyPrivacyProfileCharacteristicFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PartyPrivacyProfileCharacteristicFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for PartyPrivacyProfileCharacteristicFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
