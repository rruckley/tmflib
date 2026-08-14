use serde::{Deserialize, Serialize};
///A base / value business entity used to represent money
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Money {
    ///Currency (ISO4217 norm uses 3 letters to define the currency)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    ///A signed floating point number, the meaning of the sign is according to the context of the API that uses this Data type
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
