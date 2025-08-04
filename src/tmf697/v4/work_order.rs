//! Work Order Mobule V4

use super::{work_order_item::WorkOrderItem, MOD_PATH};
use crate::common::{note::Note, related_party::RelatedParty, tmf_error::TMFError};
use crate::tmf646::appointment::AppointmentRef;
use crate::{HasId, HasNote, HasRelatedParty, Uri, LIB_PATH};
use serde::{Deserialize, Serialize};
use tmflib_derive::{HasId, HasNote, HasRelatedParty};

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
#[derive(Clone, Debug, Default, Deserialize, HasId, HasNote, HasRelatedParty, Serialize)]
#[serde(rename_all = "camelCase")]
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
    /// use tmflib::tmf697::v4::work_order::WorkOrder;
    /// use tmflib::tmf697::v4::work_order_item::WorkOrderItem;
    /// use tmflib::tmf697::v4::work::{WorkRefOrValue,Work};
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::tmf697::v4::work::{Work, WorkRefOrValue};

    const WORK_NAME: &str = "WorkName";

    #[test]
    fn test_work_order_new() {
        let wo = WorkOrder::new();

        assert_eq!(wo.state.is_some(), true);
        assert_eq!(wo.state.unwrap(), WorkOrderStateType::default());
    }

    #[test]
    fn test_work_order_add_item() {
        let item1 = WorkOrderItem::with(WorkRefOrValue::from(Work::new(WORK_NAME)));
        let mut wo = WorkOrder::new();

        wo.add_item(item1);

        assert_eq!(wo.work_order_item.is_some(), true);

        let item2 = WorkOrderItem::with(WorkRefOrValue::from(Work::new(WORK_NAME)));

        wo.add_item(item2);

        assert_eq!(wo.work_order_item.unwrap().len(), 2);
    }
}
