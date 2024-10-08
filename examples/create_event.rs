//! Create Event Example

use tmflib::common::event::EventPayload;

#[cfg(all(feature = "tmf674", feature = "build-V4"))]
use tmflib::tmf674::geographic_site_v4::{GeographicSite,GeographicSiteEventType};
#[cfg(all(feature = "tmf674", feature = "build-V5"))]
use tmflib::tmf674::geographic_site_v5::{GeographicSite,GeographicSiteEventType};

fn main() {

    let mut site = GeographicSite::new("Chatswood Branch");

    site.status = Some("Active".into());

    let event = site.to_event(GeographicSiteEventType::GeographicSiteStatusChangeEvent)
        .path("status");

    dbg!(event);
}