//! Shopping Cart Module

use serde::{Deserialize, Serialize};

use crate::common::money::Money;
use crate::{HasId,CreateTMF,LIB_PATH,HasValidity,TimePeriod};
use tmflib_derive::{HasId,HasValidity};
use crate::common::contact::ContactMedium;
use crate::common::related_party::RelatedParty;


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
}

/// Pricing for Shopping Cart
#[derive(Clone, Debug, Default, Deserialize,PartialEq, Serialize)]
pub struct Price {
    percentage: f32,
    tax_rate: f32,
    duty_free_amount: Money,
    tax_included_amount: Money,
}

/// Shopping Cart
#[derive(Clone, Debug, Default, Deserialize, HasId,HasValidity, Serialize)]
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
    cart_total_price : Vec<CartPrice>,
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
        let mut cart = ShoppingCart::create();
        cart.cart_item = Some(vec![]);
        cart.related_party = Some(vec![]);
        cart.valid_for = Some(TimePeriod::period_days(CART_DEFAULT_VALID.into()));
        cart.cart_total_price = vec![];
        cart
    }
    /// Add item to shopping cart
    /// This function will calculate a total price and add it if not present
    pub fn add_item(&mut self, item : CartItem) {
        self.cart_item.as_mut().unwrap().push(item);
    }
    /// Add Related Party
    pub fn add_party(&mut self, party : RelatedParty) {
        self.related_party.as_mut().unwrap().push(party);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[cfg(feature = "tmf632-v4")]
    use crate::tmf632::organization_v4::Organization;
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