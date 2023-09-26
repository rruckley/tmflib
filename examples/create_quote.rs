//! Create Quote Example
use tmflib::tmf648::{quote::Quote, quote_item::QuoteItem};

fn main() {
    // Create a quote
    let item = QuoteItem::new();
    let mut quote = Quote::new();
    let _result = quote.add_quote(item);
    let _result = quote.with_external_id(String::from("ExternalId"));

    dbg!(&quote);
}