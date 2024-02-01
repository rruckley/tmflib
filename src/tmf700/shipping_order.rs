//! Shipping Order Module
//! 

use super::MOD_PATH;
use super::{HasId,CreateTMF,LIB_PATH};
use tmflib_derive::HasId;

use serde::{Deserialize,Serialize};

const CLASS_PATH : &str = "shippingOrder";

/// Order for shipping of tangible goods
#[derive(Clone, Debug, Default, Deserialize, HasId, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShippingOrder {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTML Reference to this object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}

impl ShippingOrder {
    /// Create new ShippingOrder
    pub fn new() -> ShippingOrder {
        ShippingOrder::create()
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
        let so = ShippingOrder::new();
        let id = so.get_id();
        let href = so.get_href();

        assert!(href.contains(&id));
    }
}