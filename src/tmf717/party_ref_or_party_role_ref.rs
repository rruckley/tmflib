use serde::{Serialize, Deserialize};

/// Party Reference or Party Role Reference
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyRefOrPartyRoleRef {}
impl std::fmt::Display for PartyRefOrPartyRoleRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
