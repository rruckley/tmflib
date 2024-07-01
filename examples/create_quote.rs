//! Create Quote Example
use tmflib::tmf648::{quote::Quote, quote_item::QuoteItem, quote_price::QuotePrice};
use tmflib::common::price::Price;

fn main() {
    // Create a quote using various components

    // First create a quote item
    let mut item = QuoteItem::new();
    // Create a price for this item
    let price = Price::new_ex(100.0);
    // Add price to QuotePrice and set period
    let quote_price = QuotePrice::new("Subscription").price(price).period("Monthly");
    // add QuotePrice to item
    item.price(quote_price);
    // Create the new Quote
    let mut quote = Quote::new();
    // Add the item to the quote
    let _result = quote.add_quote(item);
    // Set the external Id
    let _result = quote.with_external_id(String::from("EXT123"));
    
    // Create a total price for the quote
    let total_price = Price::new_ex(3600.0);
    
    // Create QuotePrice object for the total price and set period
    let quote_total_price = QuotePrice::new("Total Contract").price(total_price).period("Contract");
    // Add QuotePrice to quote
    quote.price(quote_total_price);

    dbg!(&quote);
}