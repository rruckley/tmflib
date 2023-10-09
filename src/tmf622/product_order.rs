//! Product Order Module

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::HasId;
use crate::{CreateTMFWithTime,HasLastUpdate};

// URL Path components
use super::LIB_PATH;
use super::MOD_PATH;

const PO_PATH: &str = "order";

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ProductOrder {
    id: Option<String>,
    href: Option<String>,
    order_date: Option<String>,
}

impl HasLastUpdate for ProductOrder {
    fn set_last_update(&mut self, time : String) {
        self.order_date = Some(time);
    }
}

impl CreateTMFWithTime<ProductOrder> for ProductOrder {}

impl ProductOrder {
    pub fn new() -> ProductOrder {
        ProductOrder::create_with_time()
    }
}

impl HasId for ProductOrder {
    fn generate_href(&mut self) {
        let id = self.get_id();
        let href = format!("/{}/{}/{}/{}", LIB_PATH, MOD_PATH, PO_PATH, id);    
        self.href = Some(href);
    }
    fn generate_id(&mut self) {
        let id = Uuid::new_v4().simple().to_string();
        self.id = Some(id);    
    }
    fn get_href(&mut self) -> String {
        if self.href.is_none() {
            self.generate_href();
        } 
        self.href.as_ref().unwrap().clone()   
    }
    fn get_id(&mut self) -> String {
        if self.id.is_none() {
            self.generate_id();
        }    
        self.id.as_ref().unwrap().clone()
    }
}
