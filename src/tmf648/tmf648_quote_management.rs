//! TMF648 Management Layer

use serde::{Deserialize, Serialize};

use super::quote::Quote;

/// Quote Management Layer
#[derive(Default,Deserialize, Serialize)]
pub struct TMF648QuoteManagement {
    quotes: Vec<Quote>,
}

impl TMF648QuoteManagement {
    /// Create a new instance of the quote management layer
    /// # Warning
    /// This will be moved into Platypus
    pub fn new() -> TMF648QuoteManagement {
        TMF648QuoteManagement { quotes: vec![] }
    }

    /// Add quote into the managment layer
    pub fn add_quote(&mut self, quote: Quote) -> Result<String, String> {
        self.quotes.push(quote);
        Ok(String::from("Quote Added"))
    }
}
