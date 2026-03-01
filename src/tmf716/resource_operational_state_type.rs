use serde::{Serialize, Deserialize};
///ResourceOperationalStateType enumerations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResourceOperationalStateType {
    #[serde(rename = "enable")]
    Enable,
    #[serde(rename = "disable")]
    Disable,
}
