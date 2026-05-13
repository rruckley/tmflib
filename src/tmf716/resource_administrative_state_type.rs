use serde::{Deserialize, Serialize};
///`ResourceAdministrativeStateType` enumerations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResourceAdministrativeStateType {
    ///The resource is in an unlocked state and can be used.
    #[serde(rename = "locked")]
    Locked,
    ///The resource is in an unlocked state and can be used.
    #[serde(rename = "unlocked")]
    Unlocked,
    ///The resource is in a shutdown state and cannot be used.
    #[serde(rename = "shutdown")]
    Shutdown,
}
