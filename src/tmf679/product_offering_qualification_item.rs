//! Product Offering Qualificaiton Item Module
//! 
//! 
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "v4")]
use crate::tmf620::product_offering::ProductOfferingRef;
#[cfg(feature = "v5")]
use crate::tmf620::product_offering_v5::ProductOfferingRef;
use crate::common::note::Note;

use super::product_qualification::TaskStateType;

/// Action for this product offering
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ProductActionType {
    /// Add a new offering
    Add,
    /// Modify an exiting offering
    Modify,
    /// Delete an offering
    Delete,
    /// Make no change
    #[default]
    NoChange,
}

/// Product Offering Qualification
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductOfferingQualificationItem {
    /// Action for this qualification
    pub action: ProductActionType,
    /// Unique Id for this qualification
    pub id: Option<String>,
    /// Notes for this qualification
    pub note: Vec<Note>,
    /// Status of this qualification
    pub state: TaskStateType,
    /// Product Offerings in scope for this qualification item
    pub product_offering: Option<ProductOfferingRef>,
}

impl ProductOfferingQualificationItem {
    /// Create a new Product Offering Qualification Item
    pub fn new() -> ProductOfferingQualificationItem {
        let id = Uuid::new_v4().simple().to_string();
        ProductOfferingQualificationItem { id: Some(id), ..Default::default() }
    }
}