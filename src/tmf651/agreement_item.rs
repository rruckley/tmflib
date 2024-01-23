//! Agreement Item Module
use serde::{Deserialize,Serialize};
use crate::{HasValidity,TimePeriod};
use tmflib_derive::HasValidity;

/// Agreement Item
#[derive(Clone,Default,Debug, Deserialize, Serialize)]
pub struct AgreementItem {
    term_or_condition: Option<Vec<AgreementTermOrCondition>>,
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