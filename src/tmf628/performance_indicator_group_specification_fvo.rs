use serde::{Serialize, Deserialize};
use super::{
    AddressableFvo, ExtensibleFvo, PerformanceIndicatorSpecificationRefOrValueFvo,
};

/// Performance Indicator Group Specification FVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceIndicatorGroupSpecificationFvo {
    ///Base schema for addressable entities
    #[serde(flatten)]
    pub addressable_fvo: AddressableFvo,
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    ///A grouping or set of PerformanceIndicatorGroupSpecifications that are classified together because of common characteristics, such as technology specific, service specific, or technology/service independent.
    #[serde(rename = "groupCategory")]
    pub group_category: String,
    ///A word, term, or phrase by which a PerformanceIndicatorGroupSpecification is tagged.
    pub name: String,
    ///Reference to Performance Indicator Specifications included in this Performance Indicator Group Specification
    #[serde(rename = "performanceIndicatorSpecification")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performance_indicator_specification: Vec<
        PerformanceIndicatorSpecificationRefOrValueFvo,
    >,
}
impl std::fmt::Display for PerformanceIndicatorGroupSpecificationFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PerformanceIndicatorGroupSpecificationFvo {
    type Target = AddressableFvo;
    fn deref(&self) -> &Self::Target {
        &self.addressable_fvo
    }
}
impl std::ops::DerefMut for PerformanceIndicatorGroupSpecificationFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.addressable_fvo
    }
}
