use serde::{Deserialize, Serialize};
///Response object for Service
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceRes {}
impl std::fmt::Display for ServiceRes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
