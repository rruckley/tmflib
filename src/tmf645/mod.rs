//! TMF645 Service Qualification Module

use serde::{Deserialize, Serialize};

const TMF_MODULE: &str = "serviceQualificationManagement";

pub mod check_service_qualification;
pub mod query_service_qualification;

/// Task State Type
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum TaskStateType {
    /// Acknowledged
    #[default]
    Acknowledged,
    /// In Progress
    InProgress,
    /// Rejected
    Rejected,
    /// Terminated
    TerminatedWithError,
    /// Cancelled
    Cancelled,
    /// Done
    Done,
}

fn get_objects() -> Vec<&'static str> {
    vec![
        check_service_qualification::CLASS_PATH,
        query_service_qualification::CLASS_PATH,
    ]
}
