//! Price Module

use serde::{Deserialize, Serialize};
use super::money::Money;
use std::ops::{Add,Sub,Mul,Div};
use rust_decimal::{prelude::FromPrimitive, Decimal};

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
        let inc_dec = Decimal::from_f32(inc_price).unwrap_or_default();
        self.tax_included_amount.value = inc_dec;
        self.duty_free_amount.value = inc_dec / Decimal::from_f32(1.0 + self.tax_rate).unwrap_or_default();
        let currency_code = currency_code.unwrap_or(AUS_CURRENCY);
        let _result = self.set_currency(currency_code);
    }

    /// Set the tax exclusive price
    pub fn set_ex_price(&mut self, ex_price : f32, currency_code : Option<&str>) {
        let ex_dec = Decimal::from_f32(ex_price).unwrap_or_default();
        self.duty_free_amount.value = ex_dec;
        self.tax_included_amount.value = ex_dec * Decimal::from_f32(1.0+self.tax_rate).unwrap_or_default();
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

impl Sub for Price {
    type Output = Price;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.tax_included_amount.unit == rhs.tax_included_amount.unit {
            Price {
                percentage: self.percentage,
                tax_rate: self.tax_rate,
                tax_included_amount: self.tax_included_amount - rhs.tax_included_amount,
                duty_free_amount: self.duty_free_amount - rhs.duty_free_amount,
            }
        } else {
            self
        }
    }
}

impl Mul<f32> for Price {
    type Output = Price;

    fn mul(self, rhs: f32) -> Self::Output {
        Price {
            percentage: self.percentage,
            tax_rate: self.tax_rate,
            tax_included_amount: self.tax_included_amount * rhs,
            duty_free_amount: self.duty_free_amount * rhs,
        }
        
    }
}

impl Mul<i32> for Price {
    type Output = Price;

    fn mul(self, rhs: i32) -> Self::Output {
        Price {
            percentage: self.percentage,
            tax_rate: self.tax_rate,
            tax_included_amount: self.tax_included_amount * rhs as f32,
            duty_free_amount: self.duty_free_amount * rhs as f32,
        }
        
    }
}

impl Div for Price {
    type Output = Price;

    fn div(self, rhs: Self) -> Self::Output {
        if self.tax_included_amount.unit == rhs.tax_included_amount.unit {
            Price {
                percentage: self.percentage,
                tax_rate: self.tax_rate,
                tax_included_amount: self.tax_included_amount / rhs.tax_included_amount,
                duty_free_amount: self.duty_free_amount / rhs.duty_free_amount,
            }
        } else {
            self
        }    
    }
}

impl Div<f32> for Price {
    type Output = Price;

    fn div(self, rhs: f32) -> Self::Output {
        if rhs != 0.0 {
            Price {
                percentage: self.percentage,
                tax_rate: self.tax_rate,
                tax_included_amount: self.tax_included_amount / rhs,
                duty_free_amount: self.duty_free_amount / rhs,
            }
        } else {
            self
        }    
    }
}

impl Div<i32> for Price {
    type Output = Price;

    fn div(self, rhs: i32) -> Self::Output {
        if rhs != 0 {
            Price {
                percentage: self.percentage,
                tax_rate: self.tax_rate,
                tax_included_amount: self.tax_included_amount / rhs,
                duty_free_amount: self.duty_free_amount / rhs,
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
        assert_eq!(price.duty_free_amount.value,Decimal::from(100)/Decimal::from_f32(1.0+price.tax_rate).unwrap_or_default());
    }

    #[test]
    fn test_price_ex() {
        let price = Price::new_ex(100.0);
        assert_eq!(price.tax_included_amount.value,Decimal::from(100)*Decimal::from_f32(1.0+price.tax_rate).unwrap_or_default());
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
        let price1 = Price::new_ex(110.0);
        let price2 = Price::new_ex(220.0);
        let price_add = price1 + price2;

        assert_eq!(price_add.duty_free_amount,Money::from(330.0));
    }

    #[test]
    fn test_price_sub() {
        let price1 = Price::new_ex(330.0);
        let price2 = Price::new_ex(220.0);
        let price_sub = price1 - price2;

        assert_eq!(price_sub.duty_free_amount,Money::from(110.0));
    }

    #[test]
    fn test_price_mul() {
        let price1 = Price::new_ex(110.0);
        let price_mul = price1 * 3.0;

        assert_eq!(price_mul.duty_free_amount,Money::from(330.0));
    }

    #[test]
    fn test_price_mul_i32() {
        let price1 = Price::new_ex(110.0);
        let price_mul = price1 * 3;

        assert_eq!(price_mul.duty_free_amount,Money::from(330.0));
    }

    #[test]
    fn test_price_div() {
        let price1 = Price::new_ex(330.0);
        
        let price_mul = price1 / 3.0;

        assert_eq!(price_mul.duty_free_amount,Money::from(110.0));
    }

    #[test]
    fn test_price_div_i32() {
        let price1 = Price::new_ex(330.0);
        
        let price_mul = price1 / 3;

        assert_eq!(price_mul.duty_free_amount,Money::from(110.0));
    }
}