//! Shipping Order Module
//! 

use super::LIB_PATH;
use super::MOD_PATH;
use super::HasId;
use super::CreateTMF;

use uuid::Uuid;
use serde::{Deserialize,Serialize};

const SHIP_PATH : &str = "shipping";

/// Order for shipping of tangible goods
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ShippingOrder {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTML Reference to this object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}
impl CreateTMF<ShippingOrder> for ShippingOrder {}

impl ShippingOrder {
    /// Create new ShippingOrder
    pub fn new() -> ShippingOrder {
        ShippingOrder::create()
    }
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

#[cfg(test)]
mod test {
    use super::ShippingOrder;
    use super::HasId;
    #[test]
    fn shipping_order_create_id() {
        // Generate shipping order, test id
        let so = ShippingOrder::new();

        assert_eq!(so.id.is_some(),true);
    }

    #[test]
    fn shipping_order_create_href() {
        let so = ShippingOrder::new();

        assert_eq!(so.href.is_some(), true);
    }

    #[test]
    fn shipping_order_create_href_matches_id() {
        let mut so = ShippingOrder::new();
        let id = so.get_id();
        let href = so.get_href();

        assert!(href.contains(&id));
    }
}