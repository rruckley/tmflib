//! Create a product template
//! 
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::category::{Category,CategoryRef};
use tmflib::tmf620::product_specification::{ProductSpecification,ProductSpecificationCharacteristic, ProductSpecificationCharacteristicValueUse};

fn main() {
    let char1 = ProductSpecificationCharacteristic::new(String::from("OptionalChar"))
    .description(String::from("This characteristic is optional."));
    let char2 = ProductSpecificationCharacteristic::new(String::from("MandatoryChar"))
    .cardinality(1,1)
    .description(String::from("This is a mandatory characteristic."));


let spec = ProductSpecification::new(String::from("MySpecification"))
    .with_charateristic(char1)
    .with_charateristic(char2);

    let mut char_value_use = ProductSpecificationCharacteristicValueUse::new(String::from("ValueRestriction"));
    char_value_use
        .with_spec(spec.clone());

    let category = Category::new(String::from("Template"));
    let offer = ProductOffering::new(String::from("ProductOffering"))
        .with_category(CategoryRef::from(&category))
        .with_specification(spec.clone())
        .with_char_value_use(char_value_use);

    //dbg!(&spec);
    let category = Category::new(String::from("Template"));
    let offer = ProductOffering::new(String::from("ProductOffering"))
        .with_category(CategoryRef::from(&category));

    let offer = ProductOffering::new(String::from("ProductOffering"));
    let category    = Category::new(String::from("Category"));
    let catref  = CategoryRef::from(&category);
    offer.with_category(CategoryRef::from(&category));


    dbg!(&offer);
}