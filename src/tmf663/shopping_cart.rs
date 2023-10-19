//! Shopping Cart Module

use serde::{Deserialize, Serialize};

use crate::{HasId,CreateTMF,LIB_PATH};

use super::MOD_PATH;
use super::cart_item::CartItem;

const CART_PATH : &str = "cart";

/// Shopping Cart
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingCart {
    /// Id
    pub id: Option<String>,
    /// HTTP Reference
    pub href: Option<String>,
    /// Cart Items
    cart_item : Vec<CartItem>,
}

impl ShoppingCart {
    /// Create a new shopping cart
    pub fn new() -> ShoppingCart {
        let cart = ShoppingCart::create();
        cart
    }
    /// Add item to shopping cart
    pub fn add_item(&mut self, item : CartItem) {
        self.cart_item.push(item);
    }
}

impl HasId for ShoppingCart {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,CART_PATH,self.get_id());
        self.href = Some(href);    
    }
    fn generate_id(&mut self) {
        let id = ShoppingCart::get_uuid();
        self.id = Some(id);
        self.generate_href();  
    }
    fn get_href(&self) -> String {
        self.href.as_ref().unwrap().clone()    
    }
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone()    
    }
}

impl CreateTMF<ShoppingCart> for ShoppingCart {}