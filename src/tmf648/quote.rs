//! Quote Module
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::quote_item::QuoteItem;
use super::MOD_PATH;
use crate::common::note::Note;
use crate::LIB_PATH;

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
    pub id: String,
    /// HTML Reference to quote
    pub href: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// External reference
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<String>,
    /// Notes for Quote
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<Vec<Note>>,
    /// Quote status
    pub state: QuoteStateType,
    quote_item: Vec<QuoteItem>,
    /// Current quote version
    pub version: String,
}

impl Quote {
    /// Create a new Product Quote
    pub fn new() -> Quote {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}", LIB_PATH, MOD_PATH, QUOTE_PATH, id);
        Quote {
            id,
            href,
            description: None,
            external_id: None,
            note: None,
            version: QUOTE_VERS.to_string(),
            state: QuoteStateType::Accepted,
            quote_item: vec![],
        }
    }

    /// Set external Id for this quote
    pub fn with_external_id(&mut self, id: String) {
        self.external_id = Some(id);
    }

    /// Add a quote item into a product quote
    pub fn add_quote(&mut self, item: QuoteItem) -> Result<String, String> {
        self.quote_item.push(item);
        Ok(String::from("Quote Item Added"))
    }

    /// Get a description for this quote
    pub fn description(&self) -> String {
        match &self.description {
            Some(d) => d.clone(),
            None => {
                format!("Quote-{}",self.id)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::QuoteStateType;
    use crate::tmf648::quote::QUOTE_VERS;

    use super::Quote;
    #[test]
    fn quote_test_new_vers() {
        let quote = Quote::new();

        assert_eq!(quote.version, QUOTE_VERS.to_string());
    }

    #[test]
    fn quote_test_new_state() {
        let quote = Quote::new();

        assert_eq!(quote.state, QuoteStateType::Accepted);
    }
}
