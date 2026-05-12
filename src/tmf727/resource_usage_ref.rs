use serde::{Serialize, Deserialize};
///ResourceUsage reference. ResourceUsage is usage event for Resource.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceUsageRef {}
impl std::fmt::Display for ResourceUsageRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
