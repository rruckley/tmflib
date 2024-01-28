//!
//! 


use tmflib::tmf648::quote::Quote;

use tmflib::TMFComponent;

fn main() {
    let quote = Quote::new();

    let _component = Quote::to_component(quote);

    //component.to_string(quote);
}