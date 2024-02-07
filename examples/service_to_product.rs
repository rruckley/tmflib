//! Convert a service specification into a product specification

use tmflib::tmf633::service_specification::{ServiceSpecification, ServiceSpecificationRef};
use tmflib::tmf633::characteristic_specification::CharacteristicSpecification;
use tmflib::tmf641::service_order::ServiceOrder;
#[cfg(feature = "tmf622-v4")]
use tmflib::tmf622::product_order_v4::ProductOrder;
#[cfg(feature = "tmf622-v5")]
use tmflib::tmf622::product_order_v5::ProductOrder;
use tmflib::tmf632::individual::Individual;
use tmflib::common::note::Note;
use tmflib::common::related_party::RelatedParty;
use tmflib::tmf641::service_order_item::{ServiceOrderItem,ServiceRefOrValue};

fn main() {
    let mut ss = ServiceSpecification::new("Access");
    let cs1 = CharacteristicSpecification::new("Bandwidth")
        .mandatory()
        .description("Maximum rate of data to be configured");
    let cs2 = CharacteristicSpecification::new("QoS")
        .optional()
        .description("Quality of Service");
    ss.add_char(cs1);
    ss.add_char(cs2);
    ss.description = Some("Access Layer component".to_string());

    let ssr = ServiceSpecificationRef::from(ss);
    let mut soi = ServiceOrderItem::default();
    soi.quantity = 1;
    let mut service = ServiceRefOrValue::default();
    service.service_specification = Some(ssr);
    soi.service= service;
    
    // Since the specification is accessed via reference which requires persistence
    // we can only demonstrate manual conversion of ServiceSpecification into ProductSpecification
    // here. Complete conversion of ServiceOrder into ProductOrder would also require the additional
    // step of migrating referenced ServiceSpecifications into ProductSpecifications and generating references
    // of the same to inject into the ProductOrderItems.

    //let ps = ProductSpecification::from(ss);

    // Create new ServiceOrder
    let mut so = ServiceOrder::new();
    // Add a sample note
    so.note.as_mut().unwrap().push(Note::new("This is a Note."));
    // Create a related party
    let ind = Individual::new("John Q. Citizen");
    // Add related party reference to ServiceOrder
    so.related_party.as_mut().unwrap().push(RelatedParty::from(&ind));
    // Set the Category
    so.category = Some("Fixed Product".to_string());
    // Set the external Id
    so.external_id = Some("PON1234983".to_string());
    // Add sample Service Order Item
    so.servce_order_item.as_mut().unwrap().push(soi);

    // Now transform the Service Order into a Product Order for downstream parties
    let po = ProductOrder::from(so);

    //dbg!(ps);

    dbg!(po);
}