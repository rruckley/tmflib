//! Sales Lead Module
//! 
use crate::common::contact::ContactMedium;
use crate::common::money::Money;
use crate::common::note::Note;
use crate::common::related_party::RelatedParty;
use crate::TimePeriod;
use crate::tmf620::{ChannelRef,MarketSegmentRef};
use crate::tmf620::category::CategoryRef;
use crate::tmf620::product_offering::ProductOfferingRef;
use crate::tmf620::product_specification::ProductSpecificationRef;

use super::MOD_PATH;
use crate::{HasId,CreateTMF,LIB_PATH};
use tmflib_derive::HasId;

use serde::{Deserialize,Serialize};

const CLASS_PATH : &str = "salesLead";

/// Sales Lead Priorities
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SalesLeadPrioityType {
    /// Low Priority
    Low,
    #[default]
    /// Medium Priority (default)
    Medium,
    /// High Priority
    High,
}

/// Sales Lead Status
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SalesLeadStateType {
    /// Accepted (default)
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

/// Sales Lead - for tracking potential sales.
#[derive(Clone,Debug,Default,Deserialize, HasId, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SalesLead {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<String>,
    //creationDate
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rating : Option<String>,
    // ReferredDate
    // statusChangeDate
    #[serde(skip_serializing_if = "Option::is_none")]
    status_change_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    estimated_revenue: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority : Option<SalesLeadPrioityType>,
    status: Option<SalesLeadStateType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prospect_contact: Option<Vec<ContactMedium>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<CategoryRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<ChannelRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    market_segment: Option<MarketSegmentRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<Vec<Note>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_offering: Option<ProductOfferingRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_specification: Option<ProductSpecificationRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_party: Option<Vec<RelatedParty>>,
}

impl SalesLead {
    /// Create a new sales lead under a given names
    pub fn new(name : impl Into<String>) -> SalesLead {
        let mut sl = SalesLead::create();
        sl.name = name.into();
        sl.status = Some(SalesLeadStateType::default());
        sl.priority = Some(SalesLeadPrioityType::default());
        sl
    }
}

#[cfg(test)]
mod test {
    use super::SalesLead;
    use super::HasId;
    const SL_NAME : &str = "My Sales Lead";
    #[test]
    fn sales_lead_create_id() {
        // Generate shipping order, test id
        let sl = SalesLead::new(SL_NAME);

        assert_eq!(sl.id.is_some(),true);
    }

    #[test]
    fn sales_lead_create_href() {
        let sl = SalesLead::new(SL_NAME);

        assert_eq!(sl.href.is_some(), true);
    }

    #[test]
    fn sales_lead_create_href_matches_id() {
        let sl = SalesLead::new(SL_NAME);
        let id = sl.get_id();
        let href = sl.get_href();

        assert!(href.contains(&id));
    }

    #[test]
    fn sales_lead_create_name() {
        let sl = SalesLead::new(SL_NAME);

        assert_eq!(sl.name,SL_NAME);
    }
}