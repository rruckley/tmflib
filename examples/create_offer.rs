//! Create a product template
//! 
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::category::{Category,CategoryRef};
use tmflib::tmf620::product_specification::{ProductSpecification,ProductSpecificationCharacteristic};

fn main() {
    let char1 = ProductSpecificationCharacteristic::new(String::from("OptionalChar"))
    .description(String::from("This characteristic is optional."));
    let char2 = ProductSpecificationCharacteristic::new(String::from("MandatoryChar"))
    .cardinality(1,1)
    .description(String::from("This is a mandatory characteristic."));


let spec = ProductSpecification::new(String::from("MySpecification"))
    .with_charateristic(char1)
    .with_charateristic(char2);

    let category = Category::new(String::from("Template"));
    let offer = ProductOffering::new(String::from("ProductOffering"))
        .with_category(CategoryRef::from(&category))
        .with_specification(spec);

    dbg!(&offer);
}