//! Service Order Item Module
//! 
use serde::{Deserialize,Serialize};

use crate::tmf646::appointment::AppointmentRef;
use crate::tmf633::service_specification::ServiceSpecificationRef;
use crate::HasDescription;
use tmflib_derive::HasDescription;

/// Service Order Item Status
#[derive(Clone,Default,Debug,Deserialize, PartialEq, Serialize)]
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
#[derive(Clone,Default,Debug,Deserialize,HasDescription, Serialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct ServiceOrderItemRelationship {
    relationship_type: String,
    order_item: Option<ServiceOrderItemRef>,
}

/// Reference to an external 
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceOrderItemRef {
    item_id: String,
    service_order_href: String,
    service_order_id: String,
}

#[cfg(test)]
mod test {
    use super::{ServiceOrderItemRelationship, ServiceOrderItemStateType, ServiceRefOrValue,ServiceOrderItemRef};

    const SOI_STATUSTYPE_JSON : &str = "\"Acknowledged\"";
    const SERVICEREF_JSON : &str = "{
        \"category\" : \"Category\",
        \"description\" : \"Description\"
    }";

    const SOI_REL_JSON : &str = "{
        \"relationshipType\" : \"Parent/Child\"
    }";

    const SOI_REF_JSON : &str = "{
        \"itemId\" : \"SOI123\",
        \"serviceOrderHref\" : \"http://example.com/tmf641/order/item/SOI123\",
        \"serviceOrderId\" : \"SO123\"  
    }";

    #[test]
    fn test_soi_statustype_deseralize() {
        let soi_statustype : ServiceOrderItemStateType = serde_json::from_str(SOI_STATUSTYPE_JSON).unwrap();

        assert_eq!(soi_statustype,ServiceOrderItemStateType::Acknowledged);
    }

    #[test]
    fn test_serviceref_deserialize() {
        let serviceref : ServiceRefOrValue = serde_json::from_str(SERVICEREF_JSON).unwrap();

        assert_eq!(serviceref.category.is_some(),true);
        assert_eq!(serviceref.description.is_some(),true);

        assert_eq!(serviceref.category.unwrap().as_str(),"Category");
        assert_eq!(serviceref.description.unwrap().as_str(),"Description");
    }

    #[test]
    fn test_soi_relationship_deserialize() {
        let soi_relationship : ServiceOrderItemRelationship = serde_json::from_str(SOI_REL_JSON).unwrap();

        assert_eq!(soi_relationship.relationship_type.as_str(),"Parent/Child");
    }

    #[test]
    fn test_soiref_deserialize() {
        let soiref : ServiceOrderItemRef = serde_json::from_str(SOI_REF_JSON).unwrap();

        assert_eq!(soiref.item_id.as_str(),"SOI123");
        assert_eq!(soiref.service_order_id.as_str(),"SO123");
    }
}