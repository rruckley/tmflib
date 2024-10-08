//! Create Geograpic Site
//! 


use tmflib::tmf673::geographic_address::GeographicAddress;
#[cfg(all(feature = "tmf674", feature = "build-V4"))]
use tmflib::tmf674::geographic_site_v4::{GeographicSite, PlaceRefOrValue,CalendarPeriod};
#[cfg(all(feature = "tmf674", feature = "build-V5"))]
use tmflib::tmf674::geographic_site_v5::{GeographicSite, PlaceRefOrValue,CalendarPeriod};


fn main() {
    let address = GeographicAddress::new("HQ")
    .number("17")
    .street("Lumeah")
    .street_type("Ave")
    .suburb("Elanora Heights")
    .state("NSW");

    let site = GeographicSite::new("Home")
        .place(PlaceRefOrValue::from(address))
        .calendar(CalendarPeriod::business_hours());

    dbg!(site);
}