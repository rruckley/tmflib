//! Product Order Item Module

use serde::{Deserialize,Serialize};
use std::convert::From;

use crate::tmf620::product_offering::{ProductOffering,ProductOfferingRef};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum OrderItemActionType {
    #[default]
    Add,
    Modify,
    Delete,
    NoChange,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ProductOrderItem {
    quantity: u16,
    action: OrderItemActionType,
    product_offering : Option<ProductOfferingRef>,
}

impl From<ProductOffering> for ProductOrderItem {
    fn from(po: ProductOffering) -> Self {
        // first convert to ProductOfferRef
        let offer_ref = ProductOfferingRef::from(po);
        let mut order_item = ProductOrderItem::default();
        order_item.product_offering = Some(offer_ref);
        order_item.quantity = 1;
        order_item
    }
}
