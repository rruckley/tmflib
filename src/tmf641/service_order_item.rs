//! Service Order Item Module
//! 
use serde::{Deserialize,Serialize};

use crate::tmf646::appointment::AppointmentRef;

/// Service Order Item Status
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub enum ServiceOrderItemStateType {
    /// Acknowledged
    #[default]
    Acknowledged,
    /// Item has been rejected
    Rejected,
    /// Item is awaiting processing
    Pending,
    /// Item is on hold awaiting further action
    Held,
    /// Item is being processed
    InProgress,
    /// Item has been cancelled
    Cancelled,
    /// Item has been completed
    Completed,
    /// Item processing has failed
    Failed,
    /// Cancellation is being assessed (e.g. PONR)
    AssessingCancellation,
    /// Cancellation is pending
    PendingCancellation,
    /// Item has been partially implemented
    Partial,
}

/// Service Order Item
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct ServiceOrderItem {
    id : String,
    quantity: u16,
    state: ServiceOrderItemStateType,
    appointment : Option<AppointmentRef>,
    service_order_item: Option<Vec<ServiceOrderItem>>,
    service_order_item_relationship: Option<Vec<ServiceOrderItemRelationship>>,
}

/// Reference to and external Service Order Item
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct ServiceOrderItemRelationship {
    relationship_type: String,
    order_item: Option<ServiceOrderItemRef>,
}

/// Reference to an external 
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct ServiceOrderItemRef {
    item_id: String,
    service_order_href: String,
    service_order_id: String,
}