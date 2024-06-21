//! Shipment Module

use crate::{
    LIB_PATH,
    DateTime,
    Uri,
    HasId,
    HasName,
    Quantity,
};
use tmflib_derive::{HasId,HasName};
use crate::common::attachment::AttachmentRefOrValue;
use serde::{Deserialize,Serialize};

use super::shipment_specification::ShipmentSpecificationRefOrValue;

use super::MOD_PATH;
const CLASS_PATH: &str = "shipment";

/// Shipment Tracking
#[derive(Clone,Default,Debug,Deserialize,HasId,HasName,Serialize)]
pub struct ShipmentTrackingRef {
    href: Option<Uri>,
    id: Option<String>,
    name: Option<String>,
}

/// Shipment
#[derive(Clone,Default,Debug,Deserialize,HasId,HasName,Serialize)]
pub struct ShipmentRefOrValue {
    /// Collection Date
    pub collection_date : String,
    /// Completion Date
    pub completion_date: DateTime,
    /// Delivery Date
    pub delivery_date: DateTime,
    /// Description
    pub description: String,
    /// Expeected Delivery Date
    pub expected_delivery_date: String, 
    /// HTTP Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name : Option<String>,
    /// Requested Delivery Date
    pub requested_delivery_date: DateTime,
    /// Status
    pub state: String,
    weight: Quantity,
    // Referenced structs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment : Option<Vec<AttachmentRefOrValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipment_specification: Option<ShipmentSpecificationRefOrValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipment_tracking: Option<ShipmentTrackingRef>,
}