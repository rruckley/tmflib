use serde::{Serialize, Deserialize};
///Intent reference, for when Intent is used by other entities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntentRef {}
impl std::fmt::Display for IntentRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
