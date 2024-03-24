//! Create Event Example

use tmflib::common::event::EventPayload;
use tmflib::tmf674::geographic_site_v4::{GeographicSite,GeographicSiteEventType};

fn main() {

    let mut site = GeographicSite::new("Chatswood Branch");

    site.status = Some("Active".into());

    let event = site.to_event(GeographicSiteEventType::GeographicSiteStatusChangeEvent)
        .path("status");

    dbg!(event);
}