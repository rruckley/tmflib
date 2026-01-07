use serde::{Serialize, Deserialize};
use super::{Entity, TimePeriod};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyCreditProfile {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Name of the credit agency giving the score
    #[serde(rename = "creditAgencyName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_agency_name: Option<String>,
    ///Type of the credit agency giving the score
    #[serde(rename = "creditAgencyType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_agency_type: Option<String>,
    ///Reference corresponding to the credit rating
    #[serde(rename = "ratingReference")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rating_reference: Option<String>,
    ///A measure of a party's creditworthiness calculated on the basis of a combination of factors such as their income and credit history
    #[serde(rename = "ratingScore")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rating_score: Option<i64>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for PartyCreditProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PartyCreditProfile {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for PartyCreditProfile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
