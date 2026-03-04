use serde::{Deserialize, Serialize};
///ResourceOperationalStateType enumerations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResourceOperationalStateType {
    ///The resource is enabled and can be used.
    #[serde(rename = "enable")]
    Enable,
    ///The resource is disabled and cannot be used.
    #[serde(rename = "disable")]
    Disable,
}
