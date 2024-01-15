//! Service Test

use serde::{Deserialize,Serialize};

use crate::{HasId,CreateTMF,TimePeriod,LIB_PATH};
use tmflib_derive::HasId;
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
#[derive(Clone,Debug,Default,Deserialize, HasId, Serialize)]
pub struct ServiceTest {
    description: Option<String>,
    end_date_time: Option<String>,
    href: Option<String>,
    id: Option<String>,
    mode: Option<String>,
    name: Option<String>,
    start_date_time: Option<String>,
    state: Option<ExecutionStateType>,
    valid_for: Option<TimePeriod>,
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