//! Product Offering Qualificaiton Item Module
//! 
//! 
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tmf620::product_offering::ProductOfferingRef;
use crate::common::note::Note;

use super::product_qualification::TaskStateType;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum ProductActionType {
    Add,
    Modify,
    Delete,
    #[default]
    NoChange,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ProductOfferingQualificationItem {
    pub action: ProductActionType,
    pub id: Option<String>,
    pub note: Vec<Note>,
    pub state: TaskStateType,
    pub product_offering: Option<ProductOfferingRef>,
}

impl ProductOfferingQualificationItem {
    pub fn new() -> ProductOfferingQualificationItem {
        let id = Uuid::new_v4().simple().to_string();
        let mut poqi = ProductOfferingQualificationItem::default();
        poqi.id = Some(id);
        poqi
    }
}