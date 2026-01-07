use super::{AddressableMvo, PerformanceIndicatorSpecificationRefOrValueMvo};
use crate::common::extensible::Extensible;
use serde::{Deserialize, Serialize};

/// Performance Indicator Group Specification MVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceIndicatorGroupSpecificationMvo {
    ///Base schema for addressable entities
    #[serde(flatten)]
    pub addressable_mvo: AddressableMvo,
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///A grouping or set of PerformanceIndicatorGroupSpecifications that are classified together because of common characteristics, such as technology specific, service specific, or technology/service independent.
    #[serde(rename = "groupCategory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_category: Option<String>,
    ///A word, term, or phrase by which a PerformanceIndicatorGroupSpecification is tagged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Reference to Performance Indicator Specifications included in this Performance Indicator Group Specification
    #[serde(rename = "performanceIndicatorSpecification")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performance_indicator_specification: Vec<PerformanceIndicatorSpecificationRefOrValueMvo>,
}
impl std::fmt::Display for PerformanceIndicatorGroupSpecificationMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PerformanceIndicatorGroupSpecificationMvo {
    type Target = AddressableMvo;
    fn deref(&self) -> &Self::Target {
        &self.addressable_mvo
    }
}
impl std::ops::DerefMut for PerformanceIndicatorGroupSpecificationMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.addressable_mvo
    }
}
