//! Price Module

use serde::{Deserialize, Serialize};
use super::money::Money;

/// Default tax rate for Australian market.
const AUS_TAX_RATE : f32 = 0.10;
const AUS_CURRENCY : &str = "AUD";

/// Common Pricing structure
#[derive(Clone, Debug, Default, Deserialize,PartialEq, Serialize)]
pub struct Price {
    /// 
    pub percentage: f32,
    /// Tax Rate 
    pub tax_rate: f32,
    /// Amount excluding taxes
    pub duty_free_amount: Money,
    /// Amount including taxes
    pub tax_included_amount: Money,
}

impl Price {
    /// Create a new Price object using a tax inclusive price
    pub fn new_inc(inc_price : f32) -> Price {
        let mut price = Price {
            tax_rate : AUS_TAX_RATE,
            ..Default::default()
        };
        price.set_inc_price(inc_price,None);
        price
    }

    /// Create a new Price object using a tax exclusive price
    pub fn new_ex(ex_price : f32) -> Price {
        let mut price = Price {
            tax_rate : AUS_TAX_RATE,
            ..Default::default()
        };
        price.set_ex_price(ex_price,None);
        let _result = price.tax_included_amount.currency(AUS_CURRENCY);
        price
    }

    fn set_currency(&mut self, currency_code : &str) -> Result<String,String> {
        let inc_result = self.tax_included_amount.currency(currency_code)?;
        let ex_result = self.duty_free_amount.currency(currency_code)?;
        Ok(format!("INC: {}, EX: {}",inc_result,ex_result))
    }

    /// Set the tax inclusive price
    pub fn set_inc_price(&mut self, inc_price : f32, currency_code : Option<&str>) {
        self.tax_included_amount.value = inc_price;
        self.duty_free_amount.value = inc_price / (1.0 + self.tax_rate);
        let currency_code = currency_code.unwrap_or(AUS_CURRENCY);
        let _result = self.set_currency(currency_code);
    }

    /// Set the tax exclusive price
    pub fn set_ex_price(&mut self, ex_price : f32, currency_code : Option<&str>) {
        self.duty_free_amount.value = ex_price;
        self.tax_included_amount.value = ex_price * (1.0+self.tax_rate);
        let currency_code = currency_code.unwrap_or(AUS_CURRENCY);
        let _result = self.set_currency(currency_code);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_price_inc() {
        let price = Price::new_inc(100.0);
        assert_eq!(price.duty_free_amount.value,100.0/(1.0+price.tax_rate));
    }

    #[test]
    fn test_price_ex() {
        let price = Price::new_ex(100.0);
        assert_eq!(price.tax_included_amount.value,100.0*(1.0+price.tax_rate));
    }
}