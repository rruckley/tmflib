//! Service Test

use serde::{Deserialize,Serialize};

use crate::{HasId,CreateTMF,HasValidity, TimePeriod,LIB_PATH, DateTime};
use tmflib_derive::{HasId,HasValidity};
use crate::common::related_party::RelatedParty;
use super::MOD_PATH;

const CLASS_PATH : &str = "serviceTest";

/// Test execution status
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
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
#[derive(Clone,Debug,Default,Deserialize, HasId, HasValidity, Serialize)]
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

    const TEST : &str = "ATest";

    #[test]
    fn test_service_test_new_name() {
        let test = ServiceTest::new(TEST);

        assert_eq!(test.name,Some(TEST.into()));
    }
}