use serde::{Serialize, Deserialize};
///ResourceAdministrativeStateType enumerations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResourceAdministrativeStateType {
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "unlocked")]
    Unlocked,
    #[serde(rename = "shutdown")]
    Shutdown,
}
