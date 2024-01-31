use serde::{Deserialize, Serialize};

// URL Path components
use crate::LIB_PATH;
use super::MOD_PATH;
use crate::{HasId,CreateTMF,DateTime};
use tmflib_derive::HasId;
use crate::common::note::Note;
use super::service_order_item::ServiceOrderItem;

const CLASS_PATH: &str = "serviceOrder";

/// Service Order Status
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum ServiceOrderStateType {
    /// Acknowledged
    #[default]
    Acknowledged,
    /// Rejected
    Rejected,
    /// Pending
    Pending,
    /// Held
    Held,
    /// InProgress
    InProgress,
    /// Cancelled
    Cancelled,
    /// Completed
    Completed,
    /// Failed
    Failed,
    /// Partial
    Partial,
    /// Assessing Cancellation
    AccessingCancellation,
    /// Pending Cancellation
    PendingCancellation,
}

/// Service Order Object
#[derive(Clone, Debug, Default, Deserialize, HasId, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceOrder {
    /// Cancellation Date
    pub cancellation_date: DateTime,
    /// Cancellation Reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    /// Order Category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category : Option<String>,
    /// Completion Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<String>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Expected Completion Date
    pub expected_completion_date: Option<String>,
    /// External Id
    pub external_id : Option<String>,
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// URi
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Notification Contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_contact: Option<String>,
    /// Order Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_date: Option<String>,
    /// Order Priority
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// Requested Completion Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_completion_date: Option<String>,
    /// Requested Start Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_start_date: Option<String>,
    /// Start Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Order Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ServiceOrderStateType>,
    /// Order Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Note>>,
    /// Service Order Items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servce_order_item: Option<Vec<ServiceOrderItem>>,
}

impl ServiceOrder {
    /// Create a new service order object
    pub fn new() -> ServiceOrder {
        ServiceOrder::create()
    }
}
