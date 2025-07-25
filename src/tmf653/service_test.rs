//! Service Test

use serde::{Deserialize,Serialize};

use crate::{HasId, HasValidity, HasName, TimePeriod,LIB_PATH, DateTime, HasRelatedParty};
use tmflib_derive::{HasId,HasName, HasValidity,HasRelatedParty};
use crate::common::related_party::RelatedParty;
use crate::common::tmf_error::TMFError;
use super::MOD_PATH;

const CLASS_PATH : &str = "serviceTest";

/// Test execution status
#[derive(Clone,Debug,Default,Deserialize, PartialEq, Serialize)] 
pub enum ExecutionStateType {
    /// Acknowledged
    #[default]
    Acknowledged,
    /// In Progress
    InProgress,
    /// Rejected
    Rejected,
    /// Pending
    Pending,
    /// Cancelled
    Cancelled,
    /// Completed
    Completed,
    /// Failed
    Failed,
}

/// Service Test
#[derive(Clone,Debug,Default,Deserialize, HasId, HasName, HasValidity, Serialize, HasRelatedParty)]
pub struct ServiceTest {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date_time: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date_time: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<ExecutionStateType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_party: Option<Vec<RelatedParty>>,
}

impl ServiceTest {
    /// Create new ServiceTest
    pub fn new(name : impl Into<String>) -> ServiceTest {
        let mut st = ServiceTest::create();
        st.name = Some(name.into());
        st
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::HasName;

    const TEST_NAME : &str = "ATest";
    const EXECSTATE_JSON : &str = "\"Acknowledged\"";
    const SERVICETEST_JSON: &str = "{
        \"name\" : \"ServiceTestName\",
        \"id\" : \"ST123\",
        \"description\" : \"Description\"
    }";

    #[test]
    fn test_service_test_new_name() {
        let test = ServiceTest::new(TEST_NAME);

        assert_eq!(test.get_name().as_str(),TEST_NAME);
    }

    #[test]
    fn test_execstate_deserialize() {
        let execstate : ExecutionStateType = serde_json::from_str(EXECSTATE_JSON).unwrap();

        assert_eq!(execstate,ExecutionStateType::Acknowledged);
    }

    #[test]
    fn test_servicetest_deserialize() {
        let servicetest : ServiceTest = serde_json::from_str(SERVICETEST_JSON).unwrap();

        assert_eq!(servicetest.get_id().as_str(),"ST123");
        assert_eq!(servicetest.get_name().as_str(),"ServiceTestName");
        assert_eq!(servicetest.description.is_some(),true);
        assert_eq!(servicetest.description.unwrap().as_str(),"Description");
    }
}