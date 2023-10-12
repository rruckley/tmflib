//! Create Geograpic Site
//! 


use tmflib::tmf673::geographic_address::GeographicAddress;
use tmflib::tmf674::geographic_site::{GeographicSite, PlaceRefOrValue};


fn main() {
    let address = GeographicAddress::new(String::from("HQ"))
    .number("17")
    .street("Lumeah")
    .street_type("Ave")
    .suburb("Elanora Heights")
    .state("NSW");

    let site = GeographicSite::new(String::from("Home"))
        .place(PlaceRefOrValue::from(address));

    dbg!(site);
}