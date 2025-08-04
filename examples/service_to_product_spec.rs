//! Convert Service Specification (TMF633) into Product Specification (TMF620)
//!

use tmflib::tmf620::product_specification::ProductSpecification;
use tmflib::tmf633::characteristic_specification::CharacteristicSpecification;
use tmflib::tmf633::service_specification::ServiceSpecification;
use tmflib::HasDescription;

fn main() {
    let char1 = CharacteristicSpecification::new("Bandwidth")
        .optional()
        .description("Bandwidth in MB/s");

    let char2 = CharacteristicSpecification::new("Weight")
        .mandatory()
        .description("Weight in kg of item");

    let mut service_spec =
        ServiceSpecification::new("Internet Service").description("Standard Internet Service");
    service_spec.add_char(char1);
    service_spec.add_char(char2);

    let product_spec = ProductSpecification::from(&service_spec);

    dbg!(product_spec);
}
