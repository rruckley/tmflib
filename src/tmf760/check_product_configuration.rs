//! Check Product Configuration Module
//!

use serde::{Deserialize, Serialize};

use super::MOD_PATH;
use crate::{HasId, LIB_PATH};
use tmflib_derive::HasId;
const CLASS_PATH: &str = "CheckProductConfiguration";

/// Configuration Check Status
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

#[cfg(test)]
mod test {
    use crate::HasId;

    use super::{CheckProductConfiguration, TaskStateType};

    const CONFIG_ID: &str = "CONFIG123";
    const CONFIG_STATE_JSON: &str = "\"Acknowledged\"";
    const CONFIG_JSON: &str = "{
        \"id\" : \"CONFIG123\",
        \"instantSync\" : true,
        \"provideAlternatives\" : true,
        \"state\" : \"Acknowledged\"
    }";

    #[test]
    fn test_taskstate_deserialize() {
        let config_state: TaskStateType = serde_json::from_str(CONFIG_STATE_JSON).unwrap();

        assert_eq!(config_state, TaskStateType::Acknowledged);
    }

    #[test]
    fn test_checkconfig_deserialize() {
        let config: CheckProductConfiguration = serde_json::from_str(CONFIG_JSON).unwrap();

        assert_eq!(config.get_id().as_str(), "CONFIG123");
        assert_eq!(config.instant_sync, true);
        assert_eq!(config.provide_alternatives, true);
    }

    #[test]
    fn test_checkconfig_hasid() {
        let mut checkconfig = CheckProductConfiguration::new();

        checkconfig.set_id(CONFIG_ID);

        assert_eq!(checkconfig.get_id().as_str(), CONFIG_ID);
    }

    #[test]
    fn test_checkconfig_new() {
        let checkconfig = CheckProductConfiguration::new();

        assert_eq!(checkconfig.id.is_some(), true);
        assert_eq!(checkconfig.href.is_some(), true);
    }
}
