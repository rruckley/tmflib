//! Create Product Specification
use tmflib::tmf620::product_specification::{ProductSpecification,ProductSpecificationCharacteristic};

fn main() {

    let char = ProductSpecification::new(String::from("MySpecification"));

    dbg!(char);
}