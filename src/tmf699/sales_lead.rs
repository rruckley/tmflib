//! Sales Lead Module
//! 
use crate::common::money::Money;
use crate::common::note::Note;
use crate::TimePeriod;
use serde::{Deserialize,Serialize};

#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub enum SalesLeadPrioityType {
    Low,
    #[default]
    Medium,
    High,
}

#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub enum SalesLeadStateType {
    #[default]
    Accepted,
    Acknowledged,
    Cancelled,
    InProgress,
    Pending,
    Rejected,
}

#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct SalesLead {
    id: Option<String>,
    href: Option<String>,
    //creationDate
    description: Option<String>,
    name: String,
    rating : Option<String>,
    // ReferredDate
    // statusChangeDate
    status_change_reason: Option<String>,
    r#type: Option<String>,
    estimated_revenue: Option<Money>,
    priority : SalesLeadPrioityType,
    status: SalesLeadStateType,
    valid_for: Option<TimePeriod>,
    note: Option<Vec<Note>>,
}