//! Product Order Item Module

use serde::{Deserialize,Serialize};
use std::convert::From;

#[cfg(feature = "v4")]
use crate::tmf620::product_offering::{ProductOffering,ProductOfferingRef};
#[cfg(feature = "v5")]
use crate::tmf620::product_offering_v5::{ProductOffering,ProductOfferingRef};

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
