//! Shipment Specification Module

use serde::{Deserialize,Serialize};
use tmflib_derive::{HasId,HasName,HasLastUpdate,HasValidity};

use crate::{
    LIB_PATH,
    Uri,
    DateTime,
    TimePeriod,
    HasId,
    HasName,
    HasLastUpdate,
    HasValidity,
};

use super::MOD_PATH;
const CLASS_PATH: &str = "shippingSpecification";

/// Shipment Specification
#[derive(Clone,Default,Debug,Deserialize,HasId,HasName,HasLastUpdate,HasValidity,Serialize)]
pub struct ShipmentSpecificationRefOrValue {
    /// Description
    pub description: String,
    /// HTTP Reference
    pub href: Option<Uri>,
    /// Unique Id
    pub id: Option<String>,
    /// Is this a bundle of specifications?
    pub is_bundle: bool,
    /// Last time updated
    pub last_update: Option<DateTime>,
    /// Status
    pub lifecycle_status: String,
    /// Name
    pub name: Option<String>,
    /// Validity
    pub valid_for: Option<TimePeriod>,
    /// Version
    pub version: String,
}