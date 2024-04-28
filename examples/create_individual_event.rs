//! Create Event rfor Individual


use tmflib::{common::event::EventPayload, tmf632::individual::{Individual, IndividualEventType}};

fn main() {

    let individual = Individual::new("John Quincy Smith");

    let event = individual.to_event(IndividualEventType::IndividualCreateEvent);

    dbg!(event);

}
