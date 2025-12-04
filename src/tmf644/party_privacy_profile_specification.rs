use super::{
    PartyPrivacyProfileSpecificationCharacteristic, PartyPrivacyRoleSpecification,
    ProductOfferingRef, RelatedPartyRefOrPartyRoleRef,
};
use crate::common::entity::Entity;
use crate::TimePeriod;
use serde::{Deserialize, Serialize};

/// Specification of a Party Privacy Profile
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyPrivacyProfileSpecification {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///A list of roles to which this specification can apply. For example: Shop Agent, Call Center Agent.
    #[serde(rename = "applicableRole")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applicable_role: Vec<PartyPrivacyRoleSpecification>,
    ///Description of the specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Date and time when the specification was last updated
    #[serde(rename = "lastUpdate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update: Option<crate::DateTime>,
    ///Lifecycle status of the specification (for example: In Design, Active, Rejected, Retired)
    #[serde(rename = "lifecycleStatus")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_status: Option<String>,
    ///Name of the specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///List of product offerings that are covered by this specification
    #[serde(rename = "productOffering")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub product_offering: Vec<ProductOfferingRef>,
    ///List of parties or party roles involved in the definition or management of the specification
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_party: Vec<RelatedPartyRefOrPartyRoleRef>,
    ///List of characteristics of the specification, whose values would typically be supplied when the profile is instantiated
    #[serde(rename = "specCharacteristic")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spec_characteristic: Vec<PartyPrivacyProfileSpecificationCharacteristic>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    ///The version of the specification, in case it is desired to maintain multiple versions of profile specifications
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for PartyPrivacyProfileSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PartyPrivacyProfileSpecification {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for PartyPrivacyProfileSpecification {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
