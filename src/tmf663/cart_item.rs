//! Shopping Cart Item

use serde::{Deserialize, Serialize};
use crate::common::note::Note;
use crate::tmf620::product_offering::ProductOfferingRef;

use std::convert::From;
use uuid::Uuid;

/// Shopping Cart Item
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CartItem {
    id: Option<String>,
    note : Vec<Note>,
    product_offering: Option<ProductOfferingRef>,
}

impl CartItem {
    pub fn add_note(&mut self, note : Note) {
        self.note.push(note);
    }
}

impl From<ProductOfferingRef> for CartItem {
    fn from(value: ProductOfferingRef) -> Self {
        let id = Uuid::new_v4().simple().to_string();
        CartItem { 
            id: Some(id), 
            product_offering: Some(value),
            note: vec![],
        }
    }
}