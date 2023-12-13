//! Create Quote Example
use tmflib::tmf648::{quote::Quote, quote_item::QuoteItem, quote_price::{Price,QuotePrice}};

fn main() {
    // Create a quote
    let mut item = QuoteItem::new();
    let price = Price::new_ex(100.0);
    let quote_price = QuotePrice::new("Subscription").price(price).period("Monthly");
    item.price(quote_price);
    let mut quote = Quote::new();
    let _result = quote.add_quote(item);
    let _result = quote.with_external_id(String::from("EXT123"));
    
    let total_price = Price::new_ex(3600.0);
    
    let quote_total_price = QuotePrice::new("Total Contract").price(total_price).period("Contract");
    quote.price(quote_total_price);

    dbg!(&quote);
}