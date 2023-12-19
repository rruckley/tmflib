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
    /// Set currency for this Money with an optional currency_code. 
    /// This code must confirm to ISO4217. If an invalid code is passed in, an 
    /// Err is returned instead.
    /// ```
    /// use tmflib::common::money::Money;
    /// 
    /// let mut money = Money::default();
    /// money.currency("AUD");
    /// money.value = 100.0;
    /// ```
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_currency() {
        let mut money = Money::default();
        let result = money.currency("AUD");
        assert_eq!(result.is_ok(),true);
        assert_eq!(money.unit,"AUD".to_string());
    }

    #[test]
    fn test_invalid_currency() {
        let mut money = Money::default();
        let result = money.currency("INVALID");
        assert_eq!(result.is_err(),true);
    }
}