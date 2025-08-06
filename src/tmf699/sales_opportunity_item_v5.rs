//! Sales Opportunity Item

use serde::{Deserialize, Serialize};

use crate::tmf629::customer::Customer;
use crate::{
    common::{money::Money, note::Note, related_party::RelatedParty},
    HasId, HasRelatedParty, TimePeriod,
    Uri, LIB_PATH,
};
use crate::common::tmf_error::TMFError;
use tmflib_derive::{HasId, HasRelatedParty};

use super::{sales_lead_v5::SalesLeadRef, sales_opportunity_v5::SalesOpportunityPriorityType};

const CLASS_PATH: &str = "salesOpportunityItem";
use super::MOD_PATH;
/// Sales Opportunity Item
#[derive(Clone, Debug, Default, Deserialize, HasId, HasRelatedParty, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SalesOpportunityItem {
    /// Unique Id of Sales Opportunity Item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HREF for API use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<SalesOpportunityPriorityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    estimated_revenue: Option<Money>,
    /// Notes
    pub note: Vec<Note>,
    /// Related Parties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
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
