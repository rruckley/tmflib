//! Quote Module
use serde::{Deserialize, Serialize};

use super::quote_item::QuoteItem;
use super::MOD_PATH;
use super::quote_price::QuotePrice;
use crate::common::note::Note;
use crate::{LIB_PATH, HasId, CreateTMF};

const QUOTE_PATH: &str = "quote";
const QUOTE_VERS: &str = "1.0";

/// Status of the quote object
#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum QuoteStateType {
    /// Quote has been rejected
    Rejected,
    /// Quote status is pending an outcome
    Pending,
    /// Quote is being processed
    InProgress,
    /// Quote has been cancelled
    Cancelled,
    /// Quote is fully approved
    Approved,
    #[default]
    /// Quote has been recieved and accepted
    Accepted,
}

/// Product Quote
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTML Reference to quote
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Quote description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// External reference
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<String>,
    /// Notes for Quote
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<Vec<Note>>,
    /// Quote status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<QuoteStateType>,
    /// Vector of quote items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_item: Option<Vec<QuoteItem>>,
    /// Current quote version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Order Submission Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_date: Option<String>,
    /// Requested Start Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Total Quote Pricing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_total_price : Option<Vec<QuotePrice>>,
}

impl Quote {
    /// Create a new Product Quote
    pub fn new() -> Quote {
        let mut quote = Quote::create();
        quote.version = Some(QUOTE_VERS.to_string());
        quote.state = Some(QuoteStateType::Accepted);
        quote.quote_item = Some(vec![]);
        quote.quote_total_price = Some(vec![]);
        quote
    }

    /// Set external Id for this quote
    pub fn with_external_id(&mut self, id: String) {
        self.external_id = Some(id);
    }

    /// Add a quote item into a product quote
    pub fn add_quote(&mut self, item: QuoteItem) -> Result<String, String> {
        self.quote_item.as_mut().unwrap().push(item);
        Ok(String::from("Quote Item Added"))
    }

    /// Add a price entry to this quote
    pub fn price(&mut self, price : QuotePrice) {
        self.quote_total_price.as_mut().unwrap().push(price);
    }

    /// Get a description for this quote
    pub fn description(&self) -> String {
        match &self.description {
            Some(d) => d.clone(),
            None => {
                format!("Quote-{}",self.get_id())
            }
        }
    }

}

impl HasId for Quote {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,QUOTE_PATH,self.get_id());
        self.href = Some(href);    
    }
    fn generate_id(&mut self) {
        let id = Quote::get_uuid();
        self.id = Some(id);
        self.generate_href();    
    }
    fn get_class() -> String {
        QUOTE_PATH.to_owned()    
    }
    fn get_href(&self) -> String {
        self.href.as_ref().unwrap().clone()    
    }
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone()
    }
}

impl CreateTMF<Quote> for Quote {}

#[cfg(test)]
mod test {
    use super::QuoteStateType;
    use crate::tmf648::quote::QUOTE_VERS;
    use crate::HasId;

    use super::Quote;
    #[test]
    fn quote_test_new_vers() {
        let quote = Quote::new();

        assert_eq!(quote.version, Some(QUOTE_VERS.to_string()));
    }

    #[test]
    fn quote_test_new_state() {
        let quote = Quote::new();

        assert_eq!(quote.state, Some(QuoteStateType::Accepted));
    }

    #[test]
    fn quote_test_description() {
        let mut quote = Quote::new();
        quote.description = Some("description".to_string());

        assert_eq!(quote.description(), "description".to_string())
    }

    #[test]
    fn quote_test_no_description() {
        let quote = Quote::new();
        
        assert_eq!(quote.description(),format!("Quote-{}",quote.get_id()));
    }
}
