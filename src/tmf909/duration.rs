use serde::{Deserialize, Serialize};
///A time interval in a given unit of time
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Duration {
    ///Time interval (number of seconds, minutes, hours, etc.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    ///Unit of time (seconds, minutes, hours, etc.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
}
impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
