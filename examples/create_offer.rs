//! Create a product template
//!

use tmflib::tmf620::category::{CategoryRef,Category};
#[cfg(all(feature = "tmf620", feature = "build-V4"))]
use tmflib::tmf620::product_offering::ProductOffering;
#[cfg(all(feature = "tmf620", feature = "build-V5"))]
use tmflib::tmf620::product_offering_v5::ProductOffering;
use tmflib::tmf620::product_specification::{
    ProductSpecification, ProductSpecificationCharacteristic,
    ProductSpecificationCharacteristicValueUse,
};

fn main() {
    let char1 = ProductSpecificationCharacteristic::new("OptionalChar")
        .description(String::from("This characteristic is optional."));
    let char2 = ProductSpecificationCharacteristic::new("MandatoryChar")
        .cardinality(1, 1)
        .description(String::from("This is a mandatory characteristic."));

    let spec = ProductSpecification::new("MySpecification")
        .with_charateristic(char1)
        .with_charateristic(char2);

    let mut char_value_use = ProductSpecificationCharacteristicValueUse::new("ValueRestriction");
    char_value_use.with_spec(spec.clone());

    let category = Category::new("Template");
    let offer = ProductOffering::new("ProductOffering")
        .with_category(CategoryRef::from(&category))
        // .with_specification(spec.clone())
        .with_char_value_use(char_value_use);

    dbg!(&offer);
}
