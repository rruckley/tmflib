//! Service Order Item Module
//! 
use serde::{Deserialize,Serialize};

use crate::tmf646::appointment::AppointmentRef;
use crate::tmf633::service_specification::ServiceSpecificationRef;

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

/// Link to Service via reference or value
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct ServiceRefOrValue {
    /// Category
    pub category: Option<String>,
    /// Description
    pub description: Option<String>,
    /// End Date
    pub end_date: Option<String>,
    /// Has Started
    pub has_started: Option<String>,
    /// Specification
    pub service_specification: Option<ServiceSpecificationRef>,
}

/// Service Order Item
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct ServiceOrderItem {
    /// Unique Id
    pub id : String,
    /// Quantity
    pub quantity: u16,
    /// Status
    pub state: ServiceOrderItemStateType,
    /// Appointment
    pub appointment : Option<AppointmentRef>,
    /// Service Order Line Items
    pub service_order_item: Option<Vec<ServiceOrderItem>>,
    /// Service Order Line Item Relationships
    pub service_order_item_relationship: Option<Vec<ServiceOrderItemRelationship>>,
    /// Service
    pub service : ServiceRefOrValue,
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