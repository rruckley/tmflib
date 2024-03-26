//! Shopping Cart Item

use serde::{Deserialize, Serialize};
use crate::common::note::Note;
#[cfg(feature = "v4")]
use crate::tmf620::product_offering::ProductOfferingRef;
#[cfg(feature = "v5")]
use crate::tmf620::product_offering_v5::ProductOfferingRef;

use std::convert::From;
use uuid::Uuid;

/// Shopping Cart Item
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CartItem {
    id: Option<String>,
    /// Notes for this Cart Item
    pub note : Vec<Note>,
    /// Product Offering in cart
    pub product_offering: Option<ProductOfferingRef>,
    /// Quantity
    pub quantity: u16,
}

impl CartItem {
    /// Add a note to this cart item
    pub fn add_note(&mut self, note : Note) {
        self.note.push(note);
    }
}

impl From<ProductOfferingRef> for CartItem {
    fn from(value: ProductOfferingRef) -> Self {
        let id = Uuid::new_v4().simple().to_string();
        CartItem { 
            id: Some(id), 
            quantity: 1,
            product_offering: Some(value),
            note: vec![],
        }
    }
}