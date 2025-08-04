//! Sales Opportunity Item

use serde::{Deserialize, Serialize};

use crate::tmf629::customer::Customer;
use crate::{
    common::{money::Money, note::Note, related_party::RelatedParty},
    HasRelatedParty, TimePeriod,
};
use tmflib_derive::HasRelatedParty;

use super::{sales_lead_v5::SalesLeadRef, sales_opportunity_v5::SalesOpportunityPriorityType};

/// Sales Opportunity Item
#[derive(Clone, Debug, Default, Deserialize, HasRelatedParty, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SalesOpportunityItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<SalesOpportunityPriorityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    estimated_revenue: Option<Money>,
    /// Notes
    pub note: Vec<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_party: Option<Vec<RelatedParty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sales_lead: Option<SalesLeadRef>,
}

impl SalesOpportunityItem {
    /// Create a new Opportunity Item
    pub fn new() -> SalesOpportunityItem {
        SalesOpportunityItem {
            valid_for: Some(TimePeriod::period_30days()),
            related_party: Some(vec![]),
            ..Default::default()
        }
    }

    /// Add customer to opportunity item
    pub fn for_customer(mut self, cust: Customer) -> SalesOpportunityItem {
        self.add_party(RelatedParty::from(&cust));
        self
    }
}
