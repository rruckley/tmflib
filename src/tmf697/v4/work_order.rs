//! Work Order Mobule V4

use crate::{
    LIB_PATH,
    HasId,
    HasNote,
    HasRelatedParty,
    Uri,
};
use super::{work_order_item::WorkOrderItem, MOD_PATH};
use crate::common::{
    note::Note,
    related_party::RelatedParty,
};
use crate::tmf646::appointment::AppointmentRef;
use tmflib_derive::{HasId,HasNote,HasRelatedParty};
use serde::{Deserialize,Serialize};

const CLASS_PATH : &str = "workorder";

/// Work Order States
#[derive(Clone,Debug,Default,Deserialize,PartialEq,Serialize)]
pub enum WorkOrderStateType {
    #[default]
    /// Acknowledged
    Acknowledged,
    /// WO is rejected
    Rejected,
    /// WO is pending an action
    Pending,
    /// WO is held
    Held,
    /// WO is in progress
    InProgress,
    /// WO has been cancelled
    Cancelled,
    /// WO has been completed
    Completed,
    /// WO has failed
    Failed,
    /// WO has been partially completed
    Partial,
    /// WO is under accessment for cancellation
    AccessingCancellation,
    /// WO is awaiting cancellation
    PendingCancellation,
}

/// Work Order
#[derive(Clone,Debug,Default,Deserialize,HasId,HasNote,HasRelatedParty,Serialize)]
pub struct WorkOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@baseType")]
    base_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@schemaLocation")]
    schema_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<Uri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    /// Work Order Status@type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<WorkOrderStateType>,
    // Referenced structures
    /// Appointment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appointment : Option<AppointmentRef>,
    /// Work Order Items
    pub work_order_item : Vec<WorkOrderItem>,
    /// Work Order Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Note>>,
    /// Related parties for party specific catalogs
    #[serde(skip_serializing_if = "Option::is_none")]
    related_party: Option<Vec<RelatedParty>>,
}

impl WorkOrder {
    /// Create a new Work Order instance
    pub fn new() -> WorkOrder {
        let mut out = WorkOrder::create();
        out.state = Some(WorkOrderStateType::default());
        out
    }

    /// Add a work order item to this WorkOrder
    pub fn add_item(&mut self, item : WorkOrderItem) {
        self.work_order_item.push(item);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_work_order_new() {
        let wo = WorkOrder::new();

        assert_eq!(wo.state.is_some(),true);
        assert_eq!(wo.state.unwrap(),WorkOrderStateType::default());
    }
}