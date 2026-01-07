use serde::{Serialize, Deserialize};
use super::{AgreementSpecificationRef, Entity, TimePeriod};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Customer360AgreementVo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "agreementPeriod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agreement_period: Option<TimePeriod>,
    #[serde(rename = "agreementSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agreement_specification: Option<AgreementSpecificationRef>,
    ///The type of the agreement. For example commercial
    #[serde(rename = "agreementType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agreement_type: Option<String>,
    ///A human-readable name for the agreement
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The current status of the agreement. Typical values are: in process, approved and rejected
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for Customer360AgreementVo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Customer360AgreementVo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Customer360AgreementVo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
