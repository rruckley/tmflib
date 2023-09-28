//! Quote Module
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::quote_item::QuoteItem;
use super::MOD_PATH;
use crate::common::note::Note;
use crate::LIB_PATH;

const QUOTE_PATH: &str = "quote";
const QUOTE_VERS: &str = "1.0";

#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
pub enum QuoteStateType {
    Rejected,
    Pending,
    InProgress,
    Cancelled,
    Approved,
    #[default]
    Accepted,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    id: String,
    href: String,
    description: Option<String>,
    external_id: Option<String>,
    note: Option<Vec<Note>>,
    state: QuoteStateType,
    quote_item: Vec<QuoteItem>,
    version: String,
}

impl Quote {
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

    pub fn add_quote(&mut self, item: QuoteItem) -> Result<String, String> {
        self.quote_item.push(item);
        Ok(String::from("Quote Item Added"))
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
