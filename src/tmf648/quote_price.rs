//! Quote Price Struct
//! 

use serde::{Deserialize,Serialize};

use crate::tmf620::product_offering_price::ProductOfferingPriceRef;
use crate::common::price::Price;

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
    pub fn new(name :impl Into<String>) -> QuotePrice {
        QuotePrice {
            name : Some(name.into()),
            ..Default::default()
        }    
    }
    /// Return the price inclusive of Tax
    pub fn inc_tax(&self) -> f32 {
        match self.price.as_ref() {
            Some(p) => p.tax_included_amount.value,
            None => 0.0,
        }
    }
    /// Return the price exclusive of Tax
    pub fn ex_tax(&self) -> f32 {
        match self.price.as_ref() {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_quote_price_none() {
        let quote_price = QuotePrice::new("MyQuotePrice");

        assert_eq!(quote_price.price.is_none(),true);
        assert_eq!(quote_price.inc_tax(),0.0);
        assert_eq!(quote_price.ex_tax(),0.0);
    }

    #[test]
    fn test_quote_price_inc() {
        const PRICE : f32 = 3600.0;
        let price = Price::new_inc(PRICE);
        let quote_price = QuotePrice::new("IncPrice")
            .price(price.clone());
        assert_eq!(quote_price.inc_tax(),PRICE);
        assert_eq!(quote_price.ex_tax(),PRICE/(1.0+price.tax_rate));
    }

    #[test]
    fn test_quote_price_ex() {
        const PRICE : f32 = 3600.0;
        let price = Price::new_ex(PRICE);
        let quote_price = QuotePrice::new("IncPrice")
            .price(price.clone());
        assert_eq!(quote_price.ex_tax(),PRICE);
        assert_eq!(quote_price.inc_tax(),PRICE*(1.0+price.tax_rate));
    }
}