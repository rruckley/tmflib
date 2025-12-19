use serde::{Deserialize, Serialize};
///Reference to a MeasurementCollectionJob
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MeasurementCollectionJobRef {}
impl std::fmt::Display for MeasurementCollectionJobRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
