//! Product Qualificaiton Module


use serde::{Deserialize,Serialize};

use crate::{HasId, LIB_PATH};
use tmflib_derive::HasId;

use super::MOD_PATH;
use super::product_offering_qualification_item::ProductOfferingQualificationItem;
use crate::common::related_party::RelatedParty;
use crate::tmf620::category::CategoryRef;
#[cfg(feature = "v4")]
use crate::tmf620::product_offering::ProductOfferingRef;
#[cfg(feature = "v5")]
use crate::tmf620::product_offering_v5::ProductOfferingRef;

const CLASS_PATH : &str = "productOfferingQualification";

/// Qualification Item Status
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskStateType {
    /// Qualification is acknowledged but not processed
    Acknowledged,
    /// Qualification has been terminated with an error
    TerminatedWithError,
    /// Qualification is in progress
    #[default]
    InProgress,
    /// Qualification has completed
    Done,
}

/// Product Offering Qualification
#[derive(Clone, Debug, Default, Deserialize, HasId, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductOfferingQualification {
    category: Option<CategoryRef>,
    id: Option<String>,
    href: Option<String>,
    state: TaskStateType,
    product_offering_qualification_item: Vec<ProductOfferingQualificationItem>,
    related_party: Vec<RelatedParty>,
}

impl ProductOfferingQualification {
    /// Create a new Product Offering Qualification from a Product Offering Reference
    pub fn new(offering_ref: Option<ProductOfferingRef>) -> ProductOfferingQualification {
        let mut poq = ProductOfferingQualification::create();
        let mut poqi = ProductOfferingQualificationItem::new();
        poqi.product_offering = offering_ref;
        poq.product_offering_qualification_item.push(poqi);
        poq
    }
    /// Add a party into this Product Offering Qualification
    pub fn add_party(&mut self, party : RelatedParty) {
        self.related_party.push(party);
    }
}

