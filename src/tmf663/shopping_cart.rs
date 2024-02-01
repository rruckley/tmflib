//! Shopping Cart Module

use serde::{Deserialize, Serialize};

use crate::{HasId,CreateTMF,LIB_PATH};
use tmflib_derive::HasId;
use crate::common::contact::ContactMedium;
use crate::common::related_party::RelatedParty;


use super::MOD_PATH;
use super::cart_item::CartItem;

const CLASS_PATH : &str = "shoppingCart";

/// Shopping Cart
#[derive(Clone, Debug, Default, Deserialize, HasId, Serialize)]
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
    /// Cart Items
    #[serde(skip_serializing_if = "Option::is_none")]
    cart_item : Option<Vec<CartItem>>,
    /// Related Party
    #[serde(skip_serializing_if = "Option::is_none")]
    related_party: Option<Vec<RelatedParty>>,
}

impl ShoppingCart {
    /// Create a new shopping cart
    pub fn new() -> ShoppingCart {
        let mut cart = ShoppingCart::create();
        cart.cart_item = Some(vec![]);
        cart.related_party = Some(vec![]);
        cart
    }
    /// Add item to shopping cart
    pub fn add_item(&mut self, item : CartItem) {
        self.cart_item.as_mut().unwrap().push(item);
    }
    /// Add Related Party
    pub fn add_party(&mut self, party : RelatedParty) {
        self.related_party.as_mut().unwrap().push(party);
    }
}