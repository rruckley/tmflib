//! Product Order Item Module

use serde::{Deserialize,Serialize};
use std::convert::From;

use crate::tmf620::product_offering::{ProductOffering,ProductOfferingRef};
use crate::tmf641::service_order_item::ServiceOrderItem;

/// Action Type for Order Items
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderItemActionType {
    /// Add Order Item [Default]
    #[default]
    Add,
    /// Modify Order Item
    Modify,
    /// Delete Order Item
    Delete,
    /// No change
    NoChange,
}

/// Line item for a Product Order
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductOrderItem {
    quantity: u16,
    action: OrderItemActionType,
    product_offering : Option<ProductOfferingRef>,
}

impl From<ProductOffering> for ProductOrderItem {
    fn from(po: ProductOffering) -> Self {
        // first convert to ProductOfferRef
        let offer_ref = ProductOfferingRef::from(po);
        ProductOrderItem { quantity: 1, product_offering: Some(offer_ref), ..Default::default() }
    }
}

impl From<ServiceOrderItem> for ProductOrderItem {
    fn from(value: ServiceOrderItem) -> Self {
        let mut poi = ProductOrderItem::default();
        let po = ProductOffering::new("Generated Offer");
        
        // Setting the specification here gets lost in the conversion into a Offer reference.
        //po.product_specification = psref;
        poi.quantity = value.quantity;
        poi.product_offering = Some(ProductOfferingRef::from(po));
        poi
    }
}
