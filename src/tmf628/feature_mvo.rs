use super::{CharacteristicMvo, FeatureRelationshipMvo, PolicyRefMvo};
use crate::common::extensible::Extensible;
use serde::{Deserialize, Serialize};

///Feature MVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureMvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///This is a list of Characteristics for a particular feature.
    #[serde(rename = "featureCharacteristic")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub feature_characteristic: Vec<CharacteristicMvo>,
    ///Collection of feature relationships
    #[serde(rename = "featureRelationship")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub feature_relationship: Vec<FeatureRelationshipMvo>,
    ///unique identifier
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
    ///Reference to manage a list of feature specification policy constraints
    #[serde(rename = "policyConstraint")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_constraint: Vec<PolicyRefMvo>,
}
impl std::fmt::Display for FeatureMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for FeatureMvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for FeatureMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
