use serde::{Serialize, Deserialize};
///An amount in a given unit
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Quantity {
    ///Numeric value in a given unit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    ///Unit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
}
impl std::fmt::Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
