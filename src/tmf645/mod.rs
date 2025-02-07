//! TMF645 Service Qualification Module

use serde::{Deserialize,Serialize};

#[cfg(feature = "build-V4")]
const MOD_PATH : &str = "serviceQualificationManagement/v4";
#[cfg(feature = "build-V5")]
const MOD_PATH : &str = "serviceQualificationManagement/v5";



pub mod check_service_qualification;
pub mod query_service_qualification;

/// Task State Type
#[derive(Clone,Debug,Default,Deserialize,Serialize,PartialEq)]
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