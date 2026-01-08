use serde::{Serialize, Deserialize};
///A Party reference
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyRef {}
impl std::fmt::Display for PartyRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
