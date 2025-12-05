use serde::{Serialize, Deserialize};
///Response object for Monitor
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitorRes {}
impl std::fmt::Display for MonitorRes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
