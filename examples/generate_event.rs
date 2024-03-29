//! Example showing TMF Event notification generated from object
//! 

use tmflib::tmf620::catalog::{Catalog,CatalogEventType};
use tmflib::common::event::EventPayload;
fn main() {
    let catalog = Catalog::new("My Catalog");
    let event = catalog.to_event(CatalogEventType::CatalogCreateEvent);

    dbg!(event);
}