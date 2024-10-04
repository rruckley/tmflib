//! Convert a service specification into a product specification

#![allow(unused_imports)]

#[cfg(feature = "tmf633-v4")]
use tmflib::tmf633::service_specification::{ServiceSpecification, ServiceSpecificationRef};
#[cfg(feature = "tmf633-v4")]
use tmflib::tmf633::characteristic_specification::CharacteristicSpecification;
#[cfg(feature = "tmf641-v4")]
use tmflib::tmf641::service_order::ServiceOrder;
#[cfg(feature = "tmf622-v4")]
use tmflib::tmf622::product_order_v4::ProductOrder;
#[cfg(feature = "tmf622-v5")]
use tmflib::tmf622::product_order_v5::ProductOrder;
#[cfg(feature = "tmf632-v4")]
use tmflib::tmf632::individual_v4::Individual;
#[cfg(feature = "tmf632-v5")]
use tmflib::tmf632::individual_v5::Individual;
use tmflib::common::note::Note;
use tmflib::common::related_party::RelatedParty;
#[cfg(feature = "tmf641-v4")]
use tmflib::tmf641::service_order_item::{ServiceOrderItem,ServiceRefOrValue};
use tmflib::{HasRelatedParty,HasNote};

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

    #[cfg(feature = "tmf641-v4")]
    {
        let ssr = ServiceSpecificationRef::from(ss);
        let mut soi = ServiceOrderItem::default();
        soi.quantity = 1;
        let mut service = ServiceRefOrValue::default();
        service.service_specification = Some(ssr);
        soi.service= service;  

         // Create new ServiceOrder
        let mut so = ServiceOrder::new();
        // Add a sample note
        so.add_note(Note::new("This is a Note."));
        // Create a related party
        let ind = Individual::new("John Q. Citizen");
        // Add related party reference to ServiceOrder
        // This should use trait functions to add a party.
        so.add_party(RelatedParty::from(&ind));
        // Set the Category
        so.category = Some("Fixed Product".to_string());
        // Set the external Id
        so.external_id = Some("PON1234983".to_string());
        // Add sample Service Order Item
        // This should use an add_item() function
        so.add_item(soi);
        // Now transform the Service Order into a Product Order for downstream parties
        let po = ProductOrder::from(so);

        //dbg!(ps);
    
        dbg!(po);
    }
}