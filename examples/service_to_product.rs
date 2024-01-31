//! Convert a service specification into a product specification

use tmflib::tmf633::service_specification::ServiceSpecification;
use tmflib::tmf633::characteristic_specification::CharacteristicSpecification;
use tmflib::tmf620::product_specification::ProductSpecification;
use tmflib::tmf641::service_order::ServiceOrder;
use tmflib::tmf622::product_order::ProductOrder;
use tmflib::tmf632::individual::Individual;
use tmflib::common::note::Note;
use tmflib::common::related_party::RelatedParty;

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

    let ps = ProductSpecification::from(ss);

    let mut so = ServiceOrder::new();
    so.note.as_mut().unwrap().push(Note::new("This is a Note."));
    let ind = Individual::new("John Q. Citizen");
    so.related_party.as_mut().unwrap().push(RelatedParty::from(&ind));
    so.category = Some("Fixed Product".to_string());
    
    so.external_id = Some("PON1234983".to_string());
    let po = ProductOrder::from(so);

    dbg!(ps);

    dbg!(po);
}