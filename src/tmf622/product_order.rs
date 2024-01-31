//! Product Order Module

use serde::{Deserialize, Serialize};

use super::HasId;
use tmflib_derive::HasId;
use crate::tmf641::service_order::ServiceOrder;
use crate::common::related_party::RelatedParty;
use crate::{CreateTMF, CreateTMFWithTime,HasLastUpdate};

// URL Path components
use super::LIB_PATH;
use super::MOD_PATH;

use super::product_order_item::ProductOrderItem;

const CLASS_PATH: &str = "productOrder";

/// ProductOrder
#[derive(Debug, Default, Deserialize, HasId, Serialize)]
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
}

impl HasLastUpdate for ProductOrder {
    fn set_last_update(&mut self, time : impl Into<String>) {
        self.order_date = Some(time.into());
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
    /// # use tmflib::tmf622::product_order::ProductOrder;
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

impl From<ServiceOrder> for ProductOrder {
    fn from(value: ServiceOrder) -> Self {
        let mut po = ProductOrder::new();
        po.order_date = value.order_date.clone();
        po    
    }
}
