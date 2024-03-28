//! Sales Opportunity Item

use serde::{Deserialize,Serialize};

use crate::{common::{money::Money, note::Note, related_party::RelatedParty}, TimePeriod};

use super::sales_opportunity_v5::SalesOpportunityPriorityType;

/// Sales Opportunity Item
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct SalesOpportunityItem {
    valid_for: Option<TimePeriod>,
    priority: Option<SalesOpportunityPriorityType>,
    estimated_revenue: Option<Money>,
    /// Notes
    pub note : Vec<Note>,  
    related_party : Vec<RelatedParty>,  
}