//!s Sales Opportunity Module V5

use serde::{Deserialize,Serialize};
use crate::{HasId,CreateTMF,LIB_PATH};
use tmflib_derive::HasId;

const CLASS_PATH : &str = "opportunity";
use super::MOD_PATH;

/// Sales Opportunity Status
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub enum SalesOpportunityStateType {
    /// Accepted
    #[default]
    Accepted,
    /// Acknowledged
    Acknowledged,
    /// Cancelled
    Cancelled,
    /// In Progress
    InProgress,
    /// Pending
    Pending,
    /// Rejected
    Rejected,
}

/// Sales Opporunity Priority
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub enum SalesOpportunityPriorityType {
    /// Low Priority
    Low,
    /// Medium Priority
    #[default]
    Medium,
    /// High Priority
    High,
}

/// Sales Opportunity
#[derive(Clone,Debug,Default,Deserialize,HasId,Serialize)]
pub struct SalesOpportunity {
    id : Option<String>,
    href : Option<String>,
    description: Option<String>,
    priority : Option<SalesOpportunityPriorityType>,
    status : Option<SalesOpportunityStateType>,
}