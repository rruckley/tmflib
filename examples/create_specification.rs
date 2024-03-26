//! Create Product Specification
use tmflib::tmf620::product_specification::{ProductSpecification,ProductSpecificationCharacteristic};

fn main() {

    let char1 = ProductSpecificationCharacteristic::new("OptionalChar")
        .description(String::from("This characteristic is optional."));
    let char2 = ProductSpecificationCharacteristic::new("MandatoryChar")
        .cardinality(1,1)
        .description(String::from("This is a mandatory characteristic."));


    let spec = ProductSpecification::new("MySpecification")
        .with_charateristic(char1)
        .with_charateristic(char2);

    dbg!(spec);
}