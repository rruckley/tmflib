//! Quote Item Module

use serde::{Deserialize,Serialize};
use uuid::Uuid;

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct QuoteItem {
    id  : String,
    quantity : u16,
    quote_item : Option<Vec<QuoteItem>>,
}

impl QuoteItem {
    pub fn new() -> QuoteItem {
        let id = Uuid::new_v4().to_string();
        QuoteItem { 
            id,
            quantity : 1,
            quote_item : None,
        }
    }
}