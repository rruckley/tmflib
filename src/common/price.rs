//! Price Module

use serde::{Deserialize, Serialize};
use super::money::Money;
use std::ops::{Add,Sub,Mul,Div};

/// Default tax rate for Australian market.
const AUS_TAX_RATE : f32 = 0.10;
const AUS_CURRENCY : &str = "AUD";

/// Common Pricing structure
#[derive(Clone, Debug, Default, Deserialize,PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    /// Percentage
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

impl Add for Price {
    type Output = Price;

    fn add(self, rhs: Self) -> Self::Output {
        // Tax included amount must have the same currency
        // We could also validate ex tax amount but they are 
        // set to gether with set_currency() function
        if self.tax_included_amount.unit == rhs.tax_included_amount.unit {
            Price {
                percentage: self.percentage,
                tax_rate: self.tax_rate,
                tax_included_amount: self.tax_included_amount + rhs.tax_included_amount,
                duty_free_amount: self.duty_free_amount + rhs.duty_free_amount,
            }
        } else {
            self   
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const PRICE_JSON : &str = "{
        \"percentage\" : 30.0,
        \"taxRate\" : 10.0,
        \"dutyFreeAmount\" : { \"unit\" : \"AUD\", \"value\" : 100.0 },
        \"taxIncludedAmount\" : { \"unit\" : \"AUD\", \"value\" : 110.0 }
    }";
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

    #[test]
    fn test_price_deserialization() {
        let price : Price = serde_json::from_str(PRICE_JSON)
            .expect("PRICE_JSON");

        assert_eq!(price.percentage, 30.0);
        assert_eq!(price.tax_rate,10.0);
   
    }

    #[test]
    fn test_price_add() {
        let price1 = Price::new_inc(110.0);
        let price2 = Price::new_inc(220.0);
        let price_add = price1 + price2;

        assert_eq!(price_add.duty_free_amount,Money::from(300.0));
    }
}