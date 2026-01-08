use serde::{Deserialize, Serialize};
///Resource reference, for when Resource is used by other entities.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceRefMvo {}
impl std::fmt::Display for ResourceRefMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
