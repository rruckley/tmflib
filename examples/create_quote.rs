//! Create Quote Example
use tmflib::tmf648::quote::Quote;

fn main() {
    // Create a quote

    let quote = Quote::new();

    dbg!(&quote);
}