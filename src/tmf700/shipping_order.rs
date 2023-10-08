//! Shipping Order Module
//! 

use super::LIB_PATH;
use super::MOD_PATH;
use super::HasId;

use uuid::Uuid;
use serde::{Deserialize,Serialize};

const SHIP_PATH : &str = "shipping";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ShippingOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

impl HasId for ShippingOrder {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,SHIP_PATH,self.id.as_ref().unwrap().clone()); 
        self.href = Some(href);
    }
    fn generate_id(&mut self) {
        let id = Uuid::new_v4().simple().to_string();
        self.id = Some(id);
        self.generate_href();    
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