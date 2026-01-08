use serde::{Deserialize, Serialize};
///ResourceAdministrativeStateType enumerations; values defined by ITU X.731: 'locked': The resource is administratively prohibited from performing services for its users; 'shutdown': Use of the resource is administratively permitted to existing instances of use only. While the system remains in the shutting down state the manager may at any time cause the managed object to revert to the unlocked state; 'unlocked': The resource is administratively permitted to perform services for its users. This is independent of its inherent operability.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResourceAdministrativeStateType {
    ///The resource is locked
    #[serde(rename = "locked")]
    Locked,
    ///The resource is unlocked
    #[serde(rename = "unlocked")]
    Unlocked,
    ///The resource is shutdown
    #[serde(rename = "shutdown")]
    Shutdown,
}
