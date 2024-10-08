//! Create Quote Example

#[cfg(all(feature = "tmf632", feature = "build-V4"))]
use tmflib::tmf632::organization_v4::Organization;
#[cfg(all(feature = "tmf632", feature = "build-V5"))]
use tmflib::tmf632::organization_v5::Organization;
use tmflib::tmf648::{quote::Quote, quote_item::QuoteItem, quote_price::QuotePrice};
use tmflib::common::price::Price;
use tmflib::common::related_party::RelatedParty;
use tmflib::tmf651::agreement::Agreement;
use tmflib::{HasName, HasRelatedParty};

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
    quote.set_name("My Quote");
    // Add the item to the quote
    let _result = quote.add_quote_item(item);
    // Set the external Id
    let _result = quote.with_external_id(String::from("EXT123"));
    
    // Create a total price for the quote
    let total_price = Price::new_ex(3600.0);
    
    // Create QuotePrice object for the total price and set period
    let quote_total_price = QuotePrice::new("Total Contract").price(total_price).period("Contract");
    // Add QuotePrice to quote
    quote.price(quote_total_price);
    // Add an organisation
    let org = Organization::new("A Customer");
    quote.add_party(RelatedParty::from(&org));
    

    let agreement = Agreement::from(&quote);

    dbg!(&agreement);
}