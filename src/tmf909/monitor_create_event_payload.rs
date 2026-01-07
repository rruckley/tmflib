use serde::{Serialize, Deserialize};
use super::Monitor;
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitorCreateEventPayload {
    ///Monitoring of resources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitor: Option<Monitor>,
}
impl std::fmt::Display for MonitorCreateEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
