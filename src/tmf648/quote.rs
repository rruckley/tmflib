//! Quote Module
use uuid::Uuid;
use serde::{Deserialize,Serialize};

use crate::LIB_PATH;
use super::MOD_PATH;

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
    version : String,
    state   : QuoteStateType,
    quote_item : Vec<QuoteItem>,
}

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct QuoteItem {

}

impl Quote {
    pub fn new() -> Quote {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,QUOTE_PATH,id);
        Quote {
            id,
            href,
            version     : QUOTE_VERS.to_string(),
            state       : QuoteStateType::Accepted,
            quote_item  : vec![],
        }
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