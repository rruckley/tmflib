use serde::{Deserialize, Serialize};
///This is enumeration for Administrative state
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AdministrativeState {
    ///The administrative state is locked
    #[serde(rename = "locked")]
    Locked,
    ///The administrative state is unlocked
    #[serde(rename = "unlocked")]
    Unlocked,
}
