//! Create Event Example

use tmflib::common::event::EventPayload;
use tmflib::tmf674::geographic_site_v4::{GeographicSite,GeographicSiteEventType};

fn main() {

    let site = GeographicSite::new("Chatswood Branch");

    let event = site.to_event(GeographicSiteEventType::GeographicSiteCreateEvent);

    dbg!(event);
}