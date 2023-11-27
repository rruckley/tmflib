//! Create Cost Example

use tmflib::tmf7xx::cost::Cost;

fn main() {
    let cost = Cost::new("MyCost");

    dbg!(cost);
}