use serde::{Deserialize, Serialize};

/// Reference to a party or party role, which may be the related party or party role itself, or a reference to it. 
/// The referenced entity may be of any type, but is typically a Party or `PartyRole`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyRefOrPartyRoleRefFvo {}
impl std::fmt::Display for PartyRefOrPartyRoleRefFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
