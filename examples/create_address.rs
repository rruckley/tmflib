//! Geographic Address Example
//! 

use tmflib::tmf673::geographic_address::GeographicAddress;

fn main() {
    let address = GeographicAddress::new("Site 1")
        .number("5")
        .street("Roseland")
        .street_type("Avenue")
        .suburb("Norhtshore")
        .state("NSW");

    dbg!(address);
}