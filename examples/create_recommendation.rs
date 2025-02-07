//! Product Recommendation Example

#[cfg(feature = "tmf680")]
use tmflib::tmf680::query_product_recommendation::QueryProductRecommendation;
use tmflib::{
    // HasId,
    // HasName,
    HasDescription,
};

fn main() {
    #[cfg(feature = "tmf680")]
    {
        let recommendation = QueryProductRecommendation::new("Sample Recommendation")
            .description("This is a description for the recommendation");

        dbg!(recommendation);
    }
}