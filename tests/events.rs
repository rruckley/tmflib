//! Test event generation
//! 

use tmflib::common::event::EventPayload;
use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::catalog::CatalogEventType;
use tmflib::HasId;

#[test]
fn test_event_product_catalog() {
    let catalog = Catalog::new("MyCatalog");

    let event = catalog.to_event(CatalogEventType::CatalogCreateEvent);

    assert_eq!(event.domain.unwrap(),Catalog::get_class());
}