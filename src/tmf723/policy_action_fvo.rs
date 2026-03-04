use serde::{Deserialize, Serialize};
/// Policy Action attributes
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyActionFvo {}
impl std::fmt::Display for PolicyActionFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
