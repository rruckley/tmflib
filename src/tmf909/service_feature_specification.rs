use serde::{Serialize, Deserialize};
use super::{
    CharacteristicSpecification, ConstraintRef, FeatureSpecificationRelationship,
};
use crate::TimePeriod;

///Specification for service features
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceFeatureSpecification {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///A URI to a JSON-Schema file that defines additional attributes and relationships
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///When sub-classing, this defines the sub-class Extensible name
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///A list of feature constraints/rules
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraint: Option<Vec<ConstraintRef>>,
    ///This is a list of characteristics for a particular feature
    #[serde(rename = "featureSpecCharacteristic")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_spec_characteristic: Option<Vec<CharacteristicSpecification>>,
    ///A dependency, exclusivity or aggratation relationship between/among feature specifications.
    #[serde(rename = "featureSpecRelationship")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_spec_relationship: Option<Vec<FeatureSpecificationRelationship>>,
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///Identifier of the feature specification. Must be locally unique within the containing specification, thus allowing direct access to the feature spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///A flag indicating if this is a feature group (true) or not (false)
    #[serde(rename = "isBundle")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_bundle: Option<bool>,
    ///A flag indicating if the feature is enabled (true) or not (false)
    #[serde(rename = "isEnabled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    ///Unique name given to the feature specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    ///Version of the feature specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for ServiceFeatureSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
