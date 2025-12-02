use serde::{Serialize, Deserialize};
use super::{Agreement, PartyPrivacyProfileCharacteristic, PartyPrivacyProfileRef};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyPrivacyAgreement {
    #[serde(flatten)]
    pub agreement: Agreement,
    ///The privacy profiles that are the subject of the agreement
    #[serde(rename = "partyPrivacyProfile")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub party_privacy_profile: Vec<PartyPrivacyProfileRef>,
    ///A list of (typically) high criticality characteristics whose chosen privacy rules are included in the agreement
    #[serde(rename = "partyPrivacyProfileCharacteristic")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub party_privacy_profile_characteristic: Vec<PartyPrivacyProfileCharacteristic>,
}
impl std::fmt::Display for PartyPrivacyAgreement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PartyPrivacyAgreement {
    type Target = Agreement;
    fn deref(&self) -> &Self::Target {
        &self.agreement
    }
}
impl std::ops::DerefMut for PartyPrivacyAgreement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.agreement
    }
}
