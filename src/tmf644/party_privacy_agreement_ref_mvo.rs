use serde::{Serialize, Deserialize};
///Reference to Party Privacy Agreement resource
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyPrivacyAgreementRefMvo {}
impl std::fmt::Display for PartyPrivacyAgreementRefMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
