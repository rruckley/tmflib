//! Sales Opportunity Item

use serde::{Deserialize,Serialize};

use crate::{common::{money::Money, note::Note, related_party::RelatedParty}, TimePeriod};

use super::{sales_lead_v5::SalesLeadRef, sales_opportunity_v5::SalesOpportunityPriorityType};

/// Sales Opportunity Item
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SalesOpportunityItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<SalesOpportunityPriorityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    estimated_revenue: Option<Money>,
    /// Notes
    pub note : Vec<Note>,  
    related_party : Vec<RelatedParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sales_lead: Option<SalesLeadRef>,
}