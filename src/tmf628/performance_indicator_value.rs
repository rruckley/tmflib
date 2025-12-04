use super::PerformanceIndicatorSpecificationRefOrValue;
use serde::{Deserialize, Serialize};
///A value of a performance indicator.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceIndicatorValue {
    ///The measurement value
    #[serde(rename = "observedValue")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_value: Option<String>,
    ///A product to be created defined by value or existing defined by reference. The polymorphic attributes @type, @schemaLocation & @referredType are related to the product entity and not the RelatedProductRefOrValue class itself
    #[serde(rename = "performanceIndicatorSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performance_indicator_specification: Option<PerformanceIndicatorSpecificationRefOrValue>,
    ///The optional tag object attached to this observed value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<serde_json::Value>,
}
impl std::fmt::Display for PerformanceIndicatorValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
