use serde::{Deserialize, Serialize};
///Base schema for addressable entities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddressableFvo {}
impl std::fmt::Display for AddressableFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
