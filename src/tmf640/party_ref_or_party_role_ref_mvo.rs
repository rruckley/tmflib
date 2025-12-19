use serde::{Deserialize, Serialize};
/// Party Reference or Party Role Reference MVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyRefOrPartyRoleRefMvo {}
impl std::fmt::Display for PartyRefOrPartyRoleRefMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
