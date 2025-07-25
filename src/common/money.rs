//! Money Module
//! 
//! # Math Functions
//! This module implements the TMF Money type and provides simple maths functions to make
//! calculations easier. 
//! # Add / Sub Limitations
//! These maths functions only work for Add and Substract if the currency is the same on both sides of the operator. 
//! If there is a difference, the LHS is returned unaltered as these functions cannot fail.
//! # Mul / Div types
//! Multiplication and Division has been implemented for both f32 and i32 types. Division by zero is not permitted
//! and will result in the LHS being returned unaltered.
//! ```
//! use rust_decimal::Decimal;
//! use tmflib::common::money::Money;
//! 
//! let unit = Money::from(10.0);
//! let qty = 5;
//! let total = unit * qty;
//! assert_eq!(total.value,Decimal::from(50));
//! ```

use serde::{Deserialize,Serialize};
use std::ops::{Add,Sub,Mul,Div,AddAssign};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use crate::common::tmf_error::TMFError;

const MONEY_DEFAULT_UNIT : &str = "AUD";

/// Money sub-resource
#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
pub struct Money {
   /// ISO4217 currency code
   pub unit : String,
   /// Value
//    pub value : f32,
//    #[serde(with = "rust_decimal::serde::float")]
   pub value : Decimal,
}

impl Money {
    /// Set currency for this Money with an optional currency_code. 
    /// This code must confirm to ISO4217. If an invalid code is passed in, an 
    /// Err is returned instead.
    /// ```
    /// use tmflib::common::money::Money;
    /// use rust_decimal::Decimal;
    /// 
    /// let mut money = Money::from(100);
    /// money.currency("AUD");
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub fn currency(&mut self, currency_code : &str) -> Result<String,TMFError> {
        let c = rust_iso4217::from_code(currency_code);
        match c {
            Some (c) => {
                self.unit = c.code.into();
                Ok(self.unit.clone())
            },
            None => Err(TMFError::CurrencyError(format!("Currency Code not found: {}", currency_code)))
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn currency(&mut self, currency_code : &str) -> Result<String,TMFError> {
        let c = rust_iso4217::from_code(currency_code);
        match c {
            Some (c) => {
                self.unit = c.code();
                Ok(self.unit.clone())
            },
            None => Err(TMFError::CurrencyError(format!("Currency Code not found: {}", currency_code)))
        }
    }
}

impl From<i32> for Money {
    fn from(value: i32) -> Self {
        Money {
            value: Decimal::from(value),
            unit: MONEY_DEFAULT_UNIT.to_string(),
        }
    }
}

impl From<f32> for Money {
    fn from(value: f32) -> Self {
        Money {
            value : Decimal::from_f32(value).unwrap_or_default(),
            unit: MONEY_DEFAULT_UNIT.to_string(),
        }
    }
}

impl Add for Money {
    type Output = Money;
    fn add(self, rhs: Self) -> Self::Output {
        if self.unit == rhs.unit {
            Self {
                unit: self.unit.clone(),
                value: self.value + rhs.value,
            }
        } else {
            self
        }
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub for Money {
    type Output = Money;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.unit == rhs.unit {
            Self {
                unit: self.unit.clone(),
                value: self.value - rhs.value,
            }
        } else {
            self
        }
    }
}

impl Mul for Money {
    type Output = Money;
    fn mul(self, rhs: Self) -> Self::Output {    

        if self.unit == rhs.unit {
            Self {
                unit: self.unit.clone(),
                value: self.value * rhs.value,
            }
        } else {
            self
        }
    }
}

impl Mul<f32> for Money {
    type Output = Money;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            unit: self.unit.clone(),
            value: self.value * Decimal::from_f32(rhs).unwrap_or_default(),
        }
    }
}

impl Mul<u32> for Money {
    type Output = Money;
    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            unit: self.unit.clone(),
            value: self.value * Decimal::from(rhs),
        }
    }
}

impl Div for Money {
    type Output = Money;

    fn div(self, rhs: Self) -> Self::Output {
        if self.unit == rhs.unit && rhs.value != Decimal::ZERO {
            Self {
                unit: self.unit.clone(),
                value: self.value / rhs.value,
            } 
        } else {
                self
            
        }
    }
}

impl Div<f32> for Money {
    type Output = Money;

    fn div(self, rhs: f32) -> Self::Output {
        let dec_val = Decimal::from_f32(rhs).unwrap_or_default();
        if dec_val != Decimal::ZERO {
            Self {
                unit: self.unit.clone(),
                value: self.value / dec_val,
            } 
        } else {
            self
        }
    }
}

impl Div<i32> for Money {
    type Output = Money;

    fn div(self, rhs: i32) -> Self::Output {
        let dec_val = Decimal::from(rhs);
        if dec_val != Decimal::ZERO {
            Self {
                unit: self.unit.clone(),
                value: self.value / dec_val,
            } 
        } else {
            self
        }
    }
}


#[cfg(test)]
mod test {
    use rust_decimal::prelude::FromPrimitive;

    use super::*;

    const MONEY_JSON : &str = "{
        \"unit\" : \"AUD\",
        \"value\" : 12.34
    }";

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

    #[test]
    fn test_money_deserialize() {
        let money : Money = serde_json::from_str(MONEY_JSON)
            .expect("MONEY_JSON");

        assert_eq!(money.unit.as_str(),"AUD");
        assert_eq!(money.value,Decimal::from_f32(12.34).unwrap_or_default());
    }

    #[test]
    fn test_money_add() {
        let money1 = Money::from(10);
        let money2 = Money::from(37);

        let money_sum = money1 + money2;

        assert_eq!(money_sum.value,Money::from(47).value);
    }

    #[test]
    fn test_money_sub() {
        let money1 = Money::from(37);
        let money2 = Money::from(10);

        let money_sub = money1 - money2;

        assert_eq!(money_sub.value,Money::from(27).value);

    }

    #[test]
    fn test_money_mul() {
        let money1 = Money::from(16);
        let money2 = Money::from(3);

        let money_mul = money1 * money2;

        assert_eq!(money_mul.value,Money::from(48).value);       
    }

    #[test]
    fn test_money_mul_f32() {
        let money1 = Money::from(16);

        let money_mul = money1 * 3.0;

        assert_eq!(money_mul.value,Money::from(48).value);       
    }

    #[test]
    fn test_money_mul_i32() {
        let money1 = Money::from(16);

        let money_mul = money1 * 3;

        assert_eq!(money_mul.value,Money::from(48).value);       
    }

    #[test]
    fn test_money_div_f32() {
        let money1 = Money::from(48);
        
        let money_div = money1 / 3.0;

        assert_eq!(money_div.value,Money::from(16).value);       
    }

    #[test]
    fn test_money_div_i32() {
        let money1 = Money::from(48);
        
        let money_div = money1 / 3;

        assert_eq!(money_div.value,Money::from(16).value);       
    }
}