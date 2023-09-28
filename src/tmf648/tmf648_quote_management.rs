use serde::{Deserialize, Serialize};

use super::quote::Quote;

#[derive(Default,Deserialize, Serialize)]
pub struct TMF648QuoteManagement {
    quotes: Vec<Quote>,
}

impl TMF648QuoteManagement {
    pub fn new() -> TMF648QuoteManagement {
        TMF648QuoteManagement { quotes: vec![] }
    }

    pub fn add_quote(&mut self, quote: Quote) -> Result<String, String> {
        self.quotes.push(quote);
        Ok(String::from("Quote Added"))
    }
}
