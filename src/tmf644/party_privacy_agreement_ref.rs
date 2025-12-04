use serde::{Deserialize, Serialize};
///Reference to Party Privacy Agreement resource
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyPrivacyAgreementRef {}
impl std::fmt::Display for PartyPrivacyAgreementRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
