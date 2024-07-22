//! Agreement Item Module
use serde::{Deserialize,Serialize};
use crate::{tmf648::quote_item::QuoteItem, HasValidity, TimePeriod};
#[cfg(feature = "tmf620-v4")]
use crate::tmf620::product_offering::ProductOfferingRef;
#[cfg(feature = "tmf620-v5")]
use crate::tmf620::product_offering_v5::ProductOfferingRef;
use tmflib_derive::HasValidity;

/// Agreement Item
#[derive(Clone,Default,Debug, Deserialize, Serialize)]
pub struct AgreementItem {
    term_or_condition: Option<Vec<AgreementTermOrCondition>>,
    product_offering: Option<Vec<ProductOfferingRef>>,
}

impl From<&QuoteItem> for AgreementItem {
    fn from(_value: &QuoteItem) -> Self {
        AgreementItem::default()
    }
}

/// Agreement Item terms and conditions
#[derive(Clone,Default,Debug, Deserialize, HasValidity, Serialize)]
pub struct AgreementTermOrCondition {
    description: String,
    id: String,
    valid_for: Option<TimePeriod>,
}

/// Agreement Item Reference
#[derive(Clone,Default,Debug, Deserialize, Serialize)]
pub struct AgreementItemRef {
    id: String,
    href: String,
    agreement_item_id: String,
    name : String,
}