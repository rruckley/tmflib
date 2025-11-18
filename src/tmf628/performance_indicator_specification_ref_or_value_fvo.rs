use serde::{Serialize, Deserialize};
///A product to be created defined by value or existing defined by reference. The polymorphic attributes @type, @schemaLocation & @referredType are related to the product entity and not the RelatedProductRefOrValue class itself
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceIndicatorSpecificationRefOrValueFvo {}
impl std::fmt::Display for PerformanceIndicatorSpecificationRefOrValueFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
