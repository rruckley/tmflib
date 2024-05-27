//! Work Order Module V5

use crate::{
    LIB_PATH,
    Uri,
};
use super::MOD_PATH;


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