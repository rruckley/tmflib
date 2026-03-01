use serde::{Serialize, Deserialize};
///ResourceUsageStateType enumerations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResourceUsageStateType {
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "busy")]
    Busy,
}
