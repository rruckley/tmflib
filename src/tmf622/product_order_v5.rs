//! Product Order Module

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::HasId;
use crate::common::related_party::RelatedParty;
use crate::{CreateTMFWithTime,HasLastUpdate};

// URL Path components
use super::LIB_PATH;
use super::MOD_PATH;

use super::product_order_item::ProductOrderItem;
use super::milestone::Milestone;

const PO_PATH: &str = "order";

/// ProductOrder
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_date: Option<String>,
    product_order_item: Vec<ProductOrderItem>,
    related_party: Vec<RelatedParty>,
    product_order_milestone : Option<Vec<Milestone>>,
}

impl HasLastUpdate for ProductOrder {
    fn set_last_update(&mut self, time : String) {
        self.order_date = Some(time);
    }
}

impl CreateTMFWithTime<ProductOrder> for ProductOrder {}

impl ProductOrder {
    /// Create a new product order via trait
    pub fn new() -> ProductOrder {
        ProductOrder::create_with_time()
    }

    /// Add an ProductOrderItem into the ProductOrder
    pub fn add_order_item(&mut self, order_item : ProductOrderItem) {
        self.product_order_item.push(order_item);
    }

    /// Add a RelatedParty into the ProductOrder
    /// # Example
    /// ```
    /// # use tmflib::tmf622::product_order_v5::ProductOrder;
    /// use tmflib::common::related_party::RelatedParty;
    /// use tmflib::tmf629::customer::Customer;
    /// use tmflib::tmf632::organization::Organization;
    /// 
    /// let organization = Organization::new(String::from("My Customer"));
    /// let customer = Customer::new(organization);
    /// let mut order = ProductOrder::new();
    /// order.add_party(RelatedParty::from(&customer));
    /// dbg!(order);
    /// ```
    pub fn add_party(&mut self, party: RelatedParty) {
        self.related_party.push(party);
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
        self.generate_href(); 
    }
    fn get_href(&self) -> String {
        self.href.as_ref().unwrap().clone()   
    }
    fn get_id(&self) -> String {    
        self.id.as_ref().unwrap().clone()
    }
    fn get_class() -> String {
        PO_PATH.to_owned()
    }
}
