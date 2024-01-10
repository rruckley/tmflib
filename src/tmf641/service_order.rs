use serde::{Deserialize, Serialize};

// URL Path components
use crate::LIB_PATH;
use super::MOD_PATH;
use crate::{HasId,CreateTMF};
use crate::common::note::Note;
use super::service_order_item::ServiceOrderItem;

const CLASS_PATH: &str = "order";

/// Service Order Object
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceOrder {
    cancellation_date: String,
    id: String,
    href: String,
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

impl HasId for ServiceOrder {
    fn generate_href(&mut self) {
        self.href = format!("{}/{}",ServiceOrder::get_class_href(),self.get_id());    
    }
    fn generate_id(&mut self) {
        let id = ServiceOrder::get_uuid();
        self.id = id;
        self.generate_href();    
    }
    fn get_class() -> String {
        CLASS_PATH.to_string()    
    }
    fn get_class_href() -> String {
        format!("/{}/{}/{}",LIB_PATH,MOD_PATH,CLASS_PATH)    
    }
    fn get_href(&self) -> String {
        self.href.clone()    
    }
    fn get_id(&self) -> String {
        self.id.clone()    
    }
}

impl CreateTMF<ServiceOrder> for ServiceOrder {}
