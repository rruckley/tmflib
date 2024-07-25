//! Product Order Item Module

use serde::{Deserialize,Serialize};
use std::convert::From;

#[cfg(feature = "v4")]
use crate::tmf620::product_offering::{ProductOffering,ProductOfferingRef};
#[cfg(feature = "v5")]
use crate::tmf620::product_offering_v5::{ProductOffering,ProductOfferingRef};
use crate::tmf641::service_order_item::ServiceOrderItem;
use crate::tmf663::cart_item::CartItem;


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

impl From<CartItem> for ProductOrderItem {
    fn from(value: CartItem) -> Self {
        // Convert a Cart item into a product order item
        ProductOrderItem {
            product_offering: value.product_offering,
            quantity: value.quantity,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tmf620::product_offering::ProductOffering;
    use crate::tmf641::service_order_item::ServiceOrderItem;
    use crate::tmf663::cart_item::CartItem;
    use super::*;
    use crate::{HasId,HasName};

    const OFFER_NAME: &str = "ProductOffering";
    #[test]
    fn test_orderitem_from_offering() {
        let offer = ProductOffering::new(OFFER_NAME);

        let item = ProductOrderItem::from(offer.clone());

        assert_eq!(item.quantity,1);
        assert_eq!(item.product_offering.is_some(),true);
        let new_offer = item.product_offering.unwrap();
        assert_eq!(new_offer.name,offer.get_name());
        assert_eq!(new_offer.id,offer.get_id());
        assert_eq!(new_offer.href,offer.get_href());
    }

    #[test]
    fn test_orderitem_from_serviceitem() {
        let service_item = ServiceOrderItem::default();
        let product_item = ProductOrderItem::from(service_item.clone());

        assert_eq!(product_item.quantity,service_item.quantity);
        assert_eq!(product_item.product_offering.is_some(),true);
        let new_offer = product_item.product_offering.unwrap();
        assert_eq!(new_offer.name,"Generated Offer".to_string());
    }

    #[test]
    fn test_orderitem_from_cartitem() {
        let _offer = ProductOffering::new(OFFER_NAME);
        let cart = CartItem::default();

        let order_item = ProductOrderItem::from(cart.clone());

        assert_eq!(cart.quantity,order_item.quantity);
        // No way to set offer on cart_item, this is matching None with None
        assert_eq!(cart.product_offering,order_item.product_offering);
    }
}
