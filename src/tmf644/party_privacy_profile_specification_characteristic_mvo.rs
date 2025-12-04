use serde::{Serialize, Deserialize};
use super::{CharacteristicSpecificationMvo, PartyRoleSpecificationRefMvo};
use crate::TimePeriod;

/// Party Privacy Profile Specification Characteristic MVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyPrivacyProfileSpecificationCharacteristicMvo {
    ///Base Characteristic Specification MVO schema for use in TMForum Open-APIs
    #[serde(flatten)]
    pub characteristic_specification_mvo: CharacteristicSpecificationMvo,
    ///A list of roles in the organization who are allowed access to this characteristic
    #[serde(rename = "allowedRole")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_role: Vec<PartyRoleSpecificationRefMvo>,
    ///Level of criticality for this characteristic of personal identifiable information (e.g. in terms of the damage if this item was breached), such as low, medium, high.
    #[serde(rename = "criticalityLevel")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criticality_level: Option<String>,
    ///Description of the characteristic
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Name of the characteristic
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Type of privacy (e.g. Internal Purpose, External Purpose, Internal Retention, External Retention)
    #[serde(rename = "privacyType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_type: Option<String>,
    ///Defines the purpose authorized or refused for the characteristic (e.g. ADMIN, INFORMATION, MARKETING, RESEARCH).
    #[serde(rename = "privacyUsagePurpose")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_usage_purpose: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for PartyPrivacyProfileSpecificationCharacteristicMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PartyPrivacyProfileSpecificationCharacteristicMvo {
    type Target = CharacteristicSpecificationMvo;
    fn deref(&self) -> &Self::Target {
        &self.characteristic_specification_mvo
    }
}
impl std::ops::DerefMut for PartyPrivacyProfileSpecificationCharacteristicMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.characteristic_specification_mvo
    }
}
