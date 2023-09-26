//! Quote Module
use uuid::Uuid;
use serde::{Deserialize,Serialize};

use crate::LIB_PATH;
use super::MOD_PATH;
use super::quote_item::QuoteItem;

const QUOTE_PATH : &str = "quote";
const QUOTE_VERS : &str = "1.0";

#[derive(Clone,Debug,Deserialize,PartialEq,Serialize)]
pub enum QuoteStateType {
    Rejected,
    Pending,
    InProgress,
    Cancelled,
    Approved,
    Accepted,
}

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct Quote {
    id      : String,
    href    : String,
    description : Option<String>,
    external_id : Option<String>,
    version : String,
    state   : QuoteStateType,
    quote_item : Vec<QuoteItem>,
}

impl Quote {
    pub fn new() -> Quote {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,QUOTE_PATH,id);
        Quote {
            id,
            href,
            description : None,
            external_id : None,
            version     : QUOTE_VERS.to_string(),
            state       : QuoteStateType::Accepted,
            quote_item  : vec![],
        }
    }

    /// Set external Id for this quote
    pub fn with_external_id(&mut self, id : String) {
        self.external_id = Some(id);
    }

    pub fn add_quote(&mut self, item : QuoteItem) -> Result<String,String> {
        self.quote_item.push(item);
        Ok(String::from("Quote Item Added"))
    }
}


#[cfg(test)]
mod test {
    use crate::tmf648::quote::QUOTE_VERS;
    use super::QuoteStateType;

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