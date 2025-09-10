//! Link remote characteristic into new characteristic

use tmflib::tmf620::product_specification::{ProductSpecification, ProductSpecificationCharacteristic, ProductSpecificationCharacteristicRelationship};

fn main() {
    #[cfg(feature = "tmf645")]
    {
        let spec_char1 = ProductSpecificationCharacteristic::new("Char1").cardinality(1, 2);
        let mut spec1 = ProductSpecification::new("Spec1")
            .with_charateristic(spec_char1);
        let spec_char2 = ProductSpecificationCharacteristic::new("Char2").cardinality(3, 4);
        let spec2 = ProductSpecification::new("Spec2")
            .with_charateristic(spec_char2);

        spec1.link_characteristic(&spec2,"Char2");

        dbg!(spec1);
    }

}