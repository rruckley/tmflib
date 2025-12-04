use serde::{Deserialize, Serialize};
///Reference to Party Privacy Profile resource
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyPrivacyProfileRefFvo {}
impl std::fmt::Display for PartyPrivacyProfileRefFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
