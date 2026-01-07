use serde::{Deserialize, Serialize};
///ResourceUsageStateType enumerations; values defined by ITU X.731: 'idle': The resource is not currently in use; 'active': The resource is in use, and has sufficient spare operating capacity to provide for additional users simultaneously; 'busy': The resource is in use, but it has no spare operating capacity to provide for additional users at this instant.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResourceUsageStateType {
    ///The resource is idle
    #[serde(rename = "idle")]
    Idle,
    ///The resource is active
    #[serde(rename = "active")]
    Active,
    ///The resource is busy
    #[serde(rename = "busy")]
    Busy,
}
