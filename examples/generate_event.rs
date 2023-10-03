//! Example showing TMF Event notification generated from object
//! 

use tmflib::tmf620::catalog::{Catalog,CatalogEventType};
use tmflib::common::event::EventPayload;
fn main() {
    let catalog = Catalog::new()
        .name(String::from("Design Catalogue"));

    let event = catalog.generate_event(CatalogEventType::CatalogCreateEvent);

    //dbg!(event);

    let event_json = serde_json::to_string(&event).unwrap();

    println!("JSON: {}",event_json);
}