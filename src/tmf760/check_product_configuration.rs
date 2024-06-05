//! Check Product Configuration Module
//! 

use serde::{Deserialize, Serialize};

use tmflib_derive::HasId;
use crate::{
    HasId,
    LIB_PATH
};
use super::MOD_PATH;
const CLASS_PATH : &str = "CheckProductConfiguration";


/// Configuration Check Status
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskStateType {
    /// Config request has been recieved and acknowledged
    #[default]
    Acknowledged,
    /// Config request is in progress
    InProgress,
    /// Config request has been rejected
    Rejected,
    /// Config request has been rejected with an error
    TerminatedWithError,
    /// Config request has been cancelled
    Cancelled,
    /// Config request has completed
    Done,
}

/// Check Product Configuration request object
#[derive(Clone, Debug, Default, Deserialize, HasId, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckProductConfiguration {
    id: Option<String>,
    href: Option<String>,
    instant_sync: bool,
    provide_alternatives: bool,
    state: TaskStateType,
}

impl CheckProductConfiguration {
    /// Create a product configuration check request
    pub fn new() -> CheckProductConfiguration {
        CheckProductConfiguration::create()
    }
}