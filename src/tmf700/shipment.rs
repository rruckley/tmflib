//! Shipment Module

use crate::{
    LIB_PATH,
    DateTime,
    Uri,
    HasId,
    HasAttachment,
    HasName,
    Quantity,
};
use tmflib_derive::{HasAttachment, HasId, HasName};
use crate::common::attachment::AttachmentRefOrValue;
use crate::common::external_identifier::ExternalIdentifier;
use serde::{Deserialize,Serialize};

use super::shipment_specification::ShipmentSpecificationRefOrValue;

use super::MOD_PATH;
const CLASS_PATH: &str = "shipment";

/// Shipment Tracking
#[derive(Clone,Default,Debug,Deserialize,HasId,HasName,Serialize)]
pub struct ShipmentTrackingRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<Uri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

/// Shipment Item Action Type
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub enum ShipmentItemActionType {
    /// Add new item
    Add,
    /// Modify existing item
    Modify,
    /// Delete existing item
    Delete,
    /// No change to existing item
    #[default]
    NoChange,
}
/**
 * Move these two into TMF687 when it gets created
 */
/// Product Stock Reference
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct ProductStockRef {
    id : String,
    href : Uri,
    name: String,
}

/// Reseved Product Stock Reference
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct ReservedProductStockRef {
    id : String,
    href : Uri,
    name: String,
}

/// Shipment Item - Individual piece of eqiupment to be delivered
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct ShipmentItem {
    /// Action
    pub action: ShipmentItemActionType,
    /// Id
    id: String,
    /// Number of items
    pub quantity: String,
    /// SKU
    pub sku : String,
    /// Weight of item
    pub weight: Quantity,
    // Referenced structs
    /// Product Stock Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_stock_ref : Option<ProductStockRef>,
    /// Product Reservation Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_reservation_ref: Option<ReservedProductStockRef>,
}



/// Shipment
#[derive(Clone,Default,Debug,Deserialize,HasId,HasAttachment,HasName,Serialize)]
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
    /// Attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment : Option<Vec<AttachmentRefOrValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipment_specification: Option<ShipmentSpecificationRefOrValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipment_tracking: Option<ShipmentTrackingRef>,
    /// Set of external identifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_identifier: Option<Vec<ExternalIdentifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Shipment Item - Individual equipment items
    pub shipment_item : Option<Vec<ShipmentItem>>,
}