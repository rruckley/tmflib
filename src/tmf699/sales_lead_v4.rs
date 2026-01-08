//! Sales Lead Module
//!
use crate::common::contact::ContactMedium;
use crate::common::money::Money;
use crate::common::note::Note;
use crate::common::related_party::RelatedParty;
use crate::common::tmf_error::TMFError;

use crate::tmf620::category::CategoryRef;
#[cfg(all(feature = "tmf620", feature = "build-V4"))]
use crate::tmf620::product_offering::ProductOfferingRef;
#[cfg(all(feature = "tmf620", feature = "build-V5"))]
use crate::tmf620::product_offering_v5::ProductOfferingRef;
use crate::tmf620::product_specification::ProductSpecificationRef;
use crate::tmf620::{ChannelRef, MarketSegmentRef};

use super::MOD_PATH;
use crate::{DateTime, HasId, HasNote, HasValidity, TimePeriod};
use tmflib_derive::{HasId, HasNote, HasValidity};

use serde::{Deserialize, Serialize};

const CLASS_PATH: &str = "salesLead";
const LEAD_VALID: u64 = 30;

/// Sales Lead Priorities
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SalesLeadPriorityType {
    /// Low Priority
    Low,
    #[default]
    /// Medium Priority (default)
    Medium,
    /// High Priority
    High,
}

/// Sales Lead Status
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, HasId, HasValidity, HasNote, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SalesLead {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_date: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rating: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    referred_date: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_change_date: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_change_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    estimated_revenue: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<SalesLeadPriorityType>,
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
    pub fn new(name: impl Into<String>) -> SalesLead {
        let mut sl = SalesLead::create();
        sl.name = name.into();
        sl.status = Some(SalesLeadStateType::default());
        sl.priority = Some(SalesLeadPriorityType::default());
        sl.valid_for = Some(TimePeriod::period_days(LEAD_VALID));
        sl
    }
}

#[cfg(test)]
mod test {
    use super::HasId;
    use super::SalesLead;
    const SL_NAME: &str = "My Sales Lead";
    #[test]
    fn sales_lead_create_id() {
        // Generate shipping order, test id
        let sl = SalesLead::new(SL_NAME);

        assert_eq!(sl.id.is_some(), true);
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

        assert_eq!(sl.name, SL_NAME);
    }
}
