//! Convert a service specification into a product specification

use tmflib::tmf633::service_specification::ServiceSpecification;
use tmflib::tmf633::characteristic_specification::CharacteristicSpecification;
use tmflib::tmf620::product_specification::ProductSpecification;
use tmflib::CreateTMF;

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

    dbg!(ps);
}