use serde::{Deserialize, Serialize};
///Service reference, for when Service is used by other entities.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceRef {}
impl std::fmt::Display for ServiceRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
