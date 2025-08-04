//! Work Order Module V5

use super::work_order_item::WorkOrderItem;
use super::MOD_PATH;
use crate::common::{note::Note, related_party::RelatedParty};
use crate::tmf646::appointment::AppointmentRef;
use crate::{HasId, Uri, LIB_PATH};
use serde::{Deserialize, Serialize};
use tmflib_derive::HasId;

const CLASS_PATH: &str = "workorder";

/// Work Order States
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
/// This module represents a collection of pieces of work to be performed to deliver an outcome, e.g. provision a service.
#[derive(Clone, Debug, Default, Deserialize, HasId, Serialize)]
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
    /// HTTP Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Work Order Status@type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<WorkOrderStateType>,
    // Referenced structures
    /// Appointment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appointment: Option<AppointmentRef>,
    /// Work Order Items
    pub work_order_item: Option<Vec<WorkOrderItem>>,
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
        // Use create() to define remainint fields
        WorkOrder {
            state: Some(WorkOrderStateType::default()),
            r#type: Some(WorkOrder::get_class()),
            base_type: Some(WorkOrder::get_class()),
            ..WorkOrder::create()
        }
    }

    /// Add a work order item to this WorkOrder
    /// ```
    /// use tmflib::tmf697::v5::work_order::WorkOrder;
    /// use tmflib::tmf697::v5::work_order_item::WorkOrderItem;
    /// use tmflib::tmf697::v5::work::{WorkRefOrValue,Work};
    ///
    /// let mut wo = WorkOrder::new();
    /// let work = Work::new("Some Work");
    /// let work_item = WorkOrderItem::with(WorkRefOrValue::from(work));
    /// wo.add_item(work_item);
    /// ```
    pub fn add_item(&mut self, item: WorkOrderItem) {
        // Safely add item
        match self.work_order_item.as_mut() {
            Some(woi) => {
                woi.push(item);
            }
            None => {
                self.work_order_item = Some(vec![item]);
            }
        }
    }
}
