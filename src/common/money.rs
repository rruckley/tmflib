//! Money Module
//! 

use serde::{Deserialize,Serialize};

/// Money sub-resource
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct Money {
   /// ISO4217 currency code
   pub unit : String,
   /// Value
   pub value : f32, 
}

impl Money {
    /// Set currency for this Money
    pub fn currency(&mut self, currency_code : &str) -> Result<String,String> {
        let c = rust_iso4217::from_code(currency_code);
        match c {
            Some (c) => {
                self.unit = c.code.into();
                Ok(c.name.into())
            },
            None => Err("Currency Code not found".into())
        }
    }
}