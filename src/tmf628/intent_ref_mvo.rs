use serde::{Deserialize, Serialize};
///Intent reference, for when Intent is used by other entities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntentRefMvo {}
impl std::fmt::Display for IntentRefMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
