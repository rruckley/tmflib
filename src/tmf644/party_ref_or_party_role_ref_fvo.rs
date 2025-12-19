use serde::{Deserialize, Serialize};
/// Party Reference or Party Role Reference MVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyRefOrPartyRoleRefFvo {}
impl std::fmt::Display for PartyRefOrPartyRoleRefFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
