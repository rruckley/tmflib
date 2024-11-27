//! Quote Price Struct
//! 

use serde::{Deserialize,Serialize};

use crate::tmf620::product_offering_price::ProductOfferingPriceRef;
use crate::common::price::Price;
use rust_decimal::Decimal;

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
    pub fn inc_tax(&self) -> Decimal {
        match self.price.as_ref() {
            Some(p) => p.tax_included_amount.value,
            None => Decimal::ZERO,
        }
    }
    /// Return the price exclusive of Tax
    pub fn ex_tax(&self) -> Decimal {
        match self.price.as_ref() {
            Some(p) => p.duty_free_amount.value,
            None => Decimal::ZERO,
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
    use rust_decimal::{prelude::FromPrimitive, Decimal};
    use rust_decimal_macros::dec;

    const QUOTE_PERIOD : &str = "period";
    const QUOTE_PRICE : Decimal = dec!(3600);

    const QUOTE_PRICE_JSON : &str = "{
        \"name\" : \"QuotePrice\",
        \"description\" : \"QuotePriceDescription\"
    }";

    #[test]
    fn test_quote_price_none() {
        let quote_price = QuotePrice::new("MyQuotePrice");

        assert_eq!(quote_price.price.is_none(),true);
        assert_eq!(quote_price.inc_tax(),Decimal::ZERO);
        assert_eq!(quote_price.ex_tax(),Decimal::ZERO);
    }

    #[test]
    fn test_quote_price_inc() {
        let price = Price::new_inc(3600.0);
        let quote_price = QuotePrice::new("IncPrice")
            .price(price.clone());
        assert_eq!(quote_price.inc_tax(),QUOTE_PRICE);
        assert_eq!(quote_price.ex_tax(),QUOTE_PRICE/ Decimal::from_f32(1.0+price.tax_rate).unwrap_or_default());
    }

    #[test]
    fn test_quote_price_ex() {
        
        let price = Price::new_ex(3600.0);
        let quote_price = QuotePrice::new("IncPrice")
            .price(price.clone());
        assert_eq!(quote_price.ex_tax(),QUOTE_PRICE);
        assert_eq!(quote_price.inc_tax(),QUOTE_PRICE*Decimal::from_f32(1.0+price.tax_rate).unwrap_or_default());
    }

    #[test]
    fn test_quote_price_period() {
        
        let price = Price::new_ex(3600.0);
        let quote_price = QuotePrice::new("IncPrice")
            .price(price.clone())
            .period(QUOTE_PERIOD); 

        assert_eq!(quote_price.recurring_charge_period.is_some(),true);
        assert_eq!(quote_price.recurring_charge_period.unwrap().as_str(),QUOTE_PERIOD);
    }

    #[test]
    fn test_quote_price_deserialize() {
        let quote_price : QuotePrice = serde_json::from_str(QUOTE_PRICE_JSON).unwrap();

        assert_eq!(quote_price.name.is_some(),true);
        assert_eq!(quote_price.description.is_some(),true);

        assert_eq!(quote_price.name.unwrap().as_str(),"QuotePrice");
        assert_eq!(quote_price.description.unwrap().as_str(),"QuotePriceDescription");
    }
}