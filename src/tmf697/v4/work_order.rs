//! Work Order Mobule V4

use crate::{
    LIB_PATH,
    HasId,
    CreateTMF,
    Uri,
};
use super::{work_order_item::WorkOrderItem, MOD_PATH};
use tmflib_derive::HasId;
use serde::{Deserialize,Serialize};

const CLASS_PATH : &str = "workorder";

/// Work Order
#[derive(Clone,Debug,Default,Deserialize,HasId,Serialize)]
pub struct WorkOrder {
    r#type: Option<String>,
    base_type: Option<String>,
    schema_location: Option<String>,
    href: Option<Uri>,
    id: Option<String>,
    // Referenced structures
    work_order_item : Vec<WorkOrderItem>,
}