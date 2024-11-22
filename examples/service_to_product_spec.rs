//! Convert Service Specification (TMF633) into Product Specification (TMF620)
//! 

use tmflib::tmf620::product_specification::ProductSpecification;
use tmflib::tmf633::service_specification::ServiceSpecification;
use tmflib::tmf633::characteristic_specification::CharacteristicSpecification;

fn main() {

    let char1 = CharacteristicSpecification::new("Char1")
        .optional()
        .description("Sample Characteristic");

    let char2 = CharacteristicSpecification::new("Weight")
        .mandatory()
        .description("Weight in kg of item");

    let mut service_spec = ServiceSpecification::new("Service Specification");
    service_spec.add_char(char1);
    service_spec.add_char(char2);

    let product_spec = ProductSpecification::from(&service_spec);

    dbg!(product_spec);
}