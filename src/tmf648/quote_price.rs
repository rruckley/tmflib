//! Quote Price Struct
//! 

use serde::{Deserialize,Serialize};

use crate::tmf620::product_offering_price::ProductOfferingPriceRef;
use crate::common::money::Money;

/// Default tax rate for Australian market.
const AUS_TAX_RATE : f32 = 0.10;
const AUS_CURRENCY : &str = "AUD";

/// Price Structure
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    percentage : f32,
    tax_rate: f32,
    duty_free_amount : Money,
    tax_included_amount : Money,
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
        self.duty_free_amount.value = inc_price / self.tax_rate;
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

/// Quote Price Structure
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotePrice {
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of price entry
    pub name : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_type : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring_charge_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_of_measure : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Pricing information
    pub price : Option<Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_offering_price : Option<ProductOfferingPriceRef>,
}

impl QuotePrice {
    /// Create a new QuotePrice object with a given name
    pub fn new(name : &str) -> QuotePrice {
        QuotePrice {
            name : Some(name.to_owned()),
            ..Default::default()
        }    
    }
    /// Return the price inclusive of Tax
    pub fn inc_tax(self) -> f32 {
        match self.price {
            Some(p) => p.tax_included_amount.value,
            None => 0.0,
        }
    }
    /// Return the price exclusive of Tax
    pub fn ex_tax(self) -> f32 {
        match self.price {
            Some(p) => p.duty_free_amount.value,
            None => 0.0,
        }
    }

    /// Add pricing to this QuotePrice
    pub fn price(mut self, price : Price) -> QuotePrice {
        self.price = Some(price);
        self
    }

    /// Set the period
    pub fn period(mut self, period : &str) -> QuotePrice {
        self.recurring_charge_period = Some(period.to_owned());
        self
    }
}