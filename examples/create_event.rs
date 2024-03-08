//! Create Event Example

use tmflib::common::event::EventPayload;
use tmflib::tmf674::geographic_site_v4::{GeographicSite,GeographicSiteEvent,GeographicSiteEventType};

fn main() {

    let site = GeographicSite::new("Chatswood Branch");
    // Do we need this step?
    let geo_event = GeographicSiteEvent {
        geographic_site: site,
    };
    // Create an event
    let event = geo_event.to_event(GeographicSiteEventType::GeographicSiteCreateEvent);

    dbg!(event);
}