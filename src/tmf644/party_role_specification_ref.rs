use serde::{Serialize, Deserialize};
///Party role specification reference. A party role specification gives additional details on the part played by a party in a given context.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyRoleSpecificationRef {}
impl std::fmt::Display for PartyRoleSpecificationRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
