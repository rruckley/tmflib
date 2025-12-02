use serde::{Serialize, Deserialize};
///Reference to Party Privacy Specification resource
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyPrivacyProfileSpecificationRefFvo {}
impl std::fmt::Display for PartyPrivacyProfileSpecificationRefFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
