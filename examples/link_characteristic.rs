//! Link remote characteristic into new characteristic

use tmflib::tmf620::product_specification::{
    ProductSpecification, ProductSpecificationCharacteristic,
};

fn main() {
    #[cfg(feature = "tmf645")]
    {
        use tmflib::TimePeriod;

        let spec_char1 =
            ProductSpecificationCharacteristic::new("Local Characteristic").cardinality(1, 2);
        let mut spec1 =
            ProductSpecification::new("Local Specification").with_charateristic(spec_char1);
        let spec_char2 = ProductSpecificationCharacteristic::new("Remote Characteristic")
            .cardinality(3, 4)
            .validity(Some(TimePeriod::period_30days()))
            .description("Only valid for 30 days".to_string());

        let spec2 =
            ProductSpecification::new("Remote Specification").with_charateristic(spec_char2);

        spec1.link_characteristic(&spec2, "Remote Characteristic");

        dbg!(spec1);
    }
}
