use serde::{Serialize, Deserialize};
///This is enumeration for Administrative state
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AdministrativeState {
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "unlocked")]
    Unlocked,
}
