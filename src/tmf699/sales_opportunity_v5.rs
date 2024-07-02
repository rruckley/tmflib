//!s Sales Opportunity Module V5

use serde::{Deserialize,Serialize};
use crate::{
    HasId, 
    TimePeriod, 
    LIB_PATH, 
    HasName, 
    HasValidity
};
use crate::common::money::Money;
use tmflib_derive::{HasId,HasName, HasValidity};

const CLASS_PATH : &str = "opportunity";
use super::sales_opportunity_item_v5::SalesOpportunityItem;
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
#[derive(Clone,Debug,Default,Deserialize,HasId,HasName,HasValidity, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SalesOpportunity {
    #[serde(skip_serializing_if = "Option::is_none")]
    id : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority : Option<SalesOpportunityPriorityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status : Option<SalesOpportunityStateType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    estimated_revenue: Option<Money>,
    sales_opportunity_item: Vec<SalesOpportunityItem>,
}

impl SalesOpportunity {
    /// Create a new Sales Opportunity
    pub fn new(name : impl Into<String>) -> SalesOpportunity {
        let mut so = SalesOpportunity::create();
        so.set_name(name);
        so.sales_opportunity_item = vec![];
        so
    }

    /// Set the name of this opportunity
    pub fn name(mut self, name: impl Into<String>) -> SalesOpportunity {
        self.set_name(name);
        self
    }

    /// Add an Opportunity Item to this Opportunity
    pub fn item(mut self, item : SalesOpportunityItem) -> SalesOpportunity {
        self.add_item(item);
        self
    }

    /// Set the description of this opportunity
    pub fn description(mut self, desc: impl Into<String>) -> SalesOpportunity {
        self.description = Some(desc.into());
        self
    }

    /// Add a new item
    pub fn add_item(&mut self, item : SalesOpportunityItem) {
        self.sales_opportunity_item.push(item);
    }
}