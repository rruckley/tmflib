//! Service Test

use serde::{Deserialize,Serialize};

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
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct ServiceTest {
    description: Option<String>,
    end_date_time: Option<String>,
    href: Option<String>,
    id: Option<String>,
    mode: Option<String>,
    name: Option<String>,
    start_date_time: Option<String>,
    state: Option<ExecutionStateType>,
}