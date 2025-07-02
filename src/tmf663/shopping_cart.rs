//! Shopping Cart Module

use serde::{Deserialize, Serialize};

use crate::{HasId, LIB_PATH,HasValidity,HasRelatedParty,TimePeriod};
use tmflib_derive::{HasId,HasValidity,HasRelatedParty};
use crate::common::contact::ContactMedium;
use crate::common::related_party::RelatedParty;
use crate::common::price::Price;


use super::MOD_PATH;
use super::cart_item::CartItem;

const CLASS_PATH : &str = "shoppingCart";
const CART_DEFAULT_VALID : u8 = 7;

/// Cart pricing
#[derive(Clone, Debug, Default, Deserialize,PartialEq, Serialize)]
pub struct CartPrice {
    description: String,
    name: String,
    price_type: String,
    recurring_charge_period: String,
    unit_of_measure: String,
    price: Option<Price>,
}

/// Shopping Cart Refernce
#[derive(Clone,Default,Debug,Serialize,Deserialize)]
pub struct ShoppingCartRef {
    id : String,
    href : String,
}

impl From<ShoppingCart> for ShoppingCartRef {
    fn from(value: ShoppingCart) -> Self {
        ShoppingCartRef {
            id : value.get_id(),
            href: value.get_href(),
        }
    }
}

/// Shopping Cart
#[derive(Clone, Debug, Default, Deserialize, HasId,HasValidity,HasRelatedParty, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingCart {
    /// Contact Medium
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_medium: Option<Vec<ContactMedium>>,
    /// Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTTP Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Validity Period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    // Referenced objects
    /// Cart Price (Total)
    cart_total_price : Option<Vec<CartPrice>>,
    /// Cart Items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_item : Option<Vec<CartItem>>,
    /// Related Party
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

impl ShoppingCart {
    /// Create a new shopping cart
    pub fn new() -> ShoppingCart {
        // let mut cart = ShoppingCart::create();
        // cart.cart_item = Some(vec![]);
        // cart.related_party = Some(vec![]);
        // cart.valid_for = Some(TimePeriod::period_days(CART_DEFAULT_VALID.into()));
        // cart.cart_total_price = None;
        // cart
        ShoppingCart {
            valid_for:  Some(TimePeriod::period_days(CART_DEFAULT_VALID.into())),
            ..Default::default()
        }
    }
    /// Add item to shopping cart
    /// This function will calculate a total price and add it if not present
    pub fn add_item(&mut self, item : CartItem) {
        match self.cart_item.as_mut() {
            Some(v) => v.push(item),
            None => self.cart_item = Some(vec![item]),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[cfg(all(feature = "tmf632", feature = "build-V4"))]
    use crate::tmf632::organization_v4::Organization;
    #[cfg(all(feature = "tmf632", feature = "build-V5"))]
    use crate::tmf632::organization_v5::Organization;
    use crate::common::related_party::RelatedParty;

    const ORG : &str = "AnOrganization";

    #[test]
    fn test_cart_add_item() {
        let mut cart = ShoppingCart::new();
        let org1 = Organization::new(ORG);
        let org2 = org1.clone();

        cart.add_party(RelatedParty::from(org1));

        // Length of related_party should be 1
        assert_eq!(cart.related_party.is_some(),true);
        assert_eq!(cart.related_party.as_ref().unwrap().len(),1);
        assert_eq!(cart.related_party.as_ref().unwrap().first(),Some(&RelatedParty::from(org2)));
    }

    #[test]
    fn test_cart_add_party() {
        let mut cart = ShoppingCart::new();
        let item1 = CartItem::default();
        let item2 = item1.clone();

        cart.add_item(item1);

        // Length of cart_item should be 1
        assert_eq!(cart.cart_item.is_some(),true);
        assert_eq!(cart.cart_item.as_ref().unwrap().len(),1);
        assert_eq!(cart.cart_item.as_ref().unwrap().first(),Some(&item2));

    }

    #[test]
    fn test_cart_valid_for() {
        let cart = ShoppingCart::new();

        assert_eq!(cart.valid_for.is_some(),true);
    }
}