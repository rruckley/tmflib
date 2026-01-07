use serde::{Serialize, Deserialize};
use super::{Characteristic, ConstraintRef, FeatureRelationship};
///Configuration feature.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Feature {
    ///This is a list of feature constraints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraint: Option<Vec<ConstraintRef>>,
    ///This is a list of Characteristics for a particular feature.
    #[serde(rename = "featureCharacteristic")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub feature_characteristic: Vec<Characteristic>,
    ///This is a list of feature relationships.
    #[serde(rename = "featureRelationship")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_relationship: Option<Vec<FeatureRelationship>>,
    ///Unique identifier of the feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///True if this is a feature group. Default is false.
    #[serde(rename = "isBundle")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_bundle: Option<bool>,
    ///True if this feature is enabled. Default is true.
    #[serde(rename = "isEnabled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    ///This is the name for the feature.
    pub name: String,
}
impl std::fmt::Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
