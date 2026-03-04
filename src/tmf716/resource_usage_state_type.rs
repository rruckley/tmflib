use serde::{Deserialize, Serialize};
///ResourceUsageStateType enumerations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResourceUsageStateType {
    ///The resource is in an idle state and can be used.
    #[serde(rename = "idle")]
    Idle,
    ///The resource is in an active state and can be used.
    #[serde(rename = "active")]
    Active,
    ///The resource is in a busy state and cannot be used.
    #[serde(rename = "busy")]
    Busy,
}
