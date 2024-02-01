use serde::{Deserialize, Serialize};

// URL Path components
use crate::LIB_PATH;
use super::MOD_PATH;
use crate::{HasId,CreateTMF,DateTime};
use tmflib_derive::HasId;
use crate::common::note::Note;
use super::service_order_item::ServiceOrderItem;

const CLASS_PATH: &str = "serviceOrder";

/// Service Order Object
#[derive(Clone, Debug, Default, Deserialize, HasId, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceOrder {
    cancellation_date: DateTime,
    id: Option<String>,
    href: Option<String>,
    description: Option<String>,
    note: Option<Vec<Note>>,
    servce_order_item: Option<Vec<ServiceOrderItem>>,
}

impl ServiceOrder {
    /// Create a new service order object
    pub fn new() -> ServiceOrder {
        ServiceOrder::create()
    }
}
