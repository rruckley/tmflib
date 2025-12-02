use serde::{Serialize, Deserialize};
///Base schema for adressable entities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddressableFvo {
    ///unique identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl std::fmt::Display for AddressableFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
