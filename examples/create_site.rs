//! Create Geograpic Site
//! 


use tmflib::tmf673::geographic_address::GeographicAddress;
#[cfg(feature = "v4")]
use tmflib::tmf674::geographic_site_v4::{GeographicSite, PlaceRefOrValue};
#[cfg(feature = "v5")]
use tmflib::tmf674::geographic_site_v5::{GeographicSite, PlaceRefOrValue};


fn main() {
    let address = GeographicAddress::new(String::from("HQ"))
    .number("17")
    .street("Lumeah")
    .street_type("Ave")
    .suburb("Elanora Heights")
    .state("NSW");

    let site = GeographicSite::new(String::from("Home"))
        .place(PlaceRefOrValue::from(address))
        .calendar(CalendarPeriod::business_hours());

    dbg!(site);
}