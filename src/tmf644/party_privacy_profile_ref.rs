use serde::{Serialize, Deserialize};
///Reference to Party Privacy Profile resource
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyPrivacyProfileRef {}
impl std::fmt::Display for PartyPrivacyProfileRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
