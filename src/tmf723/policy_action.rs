use serde::{Serialize, Deserialize};
/// Policy Action attributes
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyAction {}
impl std::fmt::Display for PolicyAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
