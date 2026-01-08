//! Create Event rfor Individual

use tmflib::common::event::EventPayload;
#[cfg(all(feature = "tmf632", feature = "build-V4"))]
use tmflib::tmf632::individual_v4::{Individual, IndividualEventType};
#[cfg(all(feature = "tmf632", feature = "build-V5"))]
use tmflib::tmf632::individual_v5::{Individual, IndividualEventType};

fn main() {
    let individual = Individual::new("John Quincy Smith");

    let event = individual.to_event(IndividualEventType::IndividualCreateEvent);

    dbg!(event);
}
