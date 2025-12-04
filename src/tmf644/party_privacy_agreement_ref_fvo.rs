use serde::{Deserialize, Serialize};
///Reference to Party Privacy Agreement resource
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyPrivacyAgreementRefFvo {}
impl std::fmt::Display for PartyPrivacyAgreementRefFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
