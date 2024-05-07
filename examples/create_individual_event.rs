//! Create Event rfor Individual

#[cfg(feature = "tmf632-v4")]
use tmflib::tmf632::individual_v4::{Individual, IndividualEventType};
#[cfg(feature = "tmf632-v5")]
use tmflib::tmf632::individual_v5::{Individual, IndividualEventType};
use tmflib::common::event::EventPayload;

fn main() {

    let individual = Individual::new("John Quincy Smith");

    let event = individual.to_event(IndividualEventType::IndividualCreateEvent);

    dbg!(event);

}
