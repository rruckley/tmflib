use serde::{Serialize, Deserialize};
///Service reference, for when Service is used by other entities.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceRefMvo {}
impl std::fmt::Display for ServiceRefMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
