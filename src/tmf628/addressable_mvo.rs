use serde::{Deserialize, Serialize};
///Base schema for addressable entities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddressableMvo {
    ///unique identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl std::fmt::Display for AddressableMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
