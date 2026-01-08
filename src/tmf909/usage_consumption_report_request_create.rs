use serde::{Serialize, Deserialize};
use super::{
    NetworkProductRef, RelatedParty, UsageConsumptionReportRef,
    UsageVolumeProductRef,
};
use crate::TimePeriod;

/// An UsageConsumptionReportRequest allows to manage the calculation request of an usage consumption report
/// Skipped properties: id,href*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageConsumptionReportRequestCreate {
    ///Reference of the buckets for which the usage consumption report is requested
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<Vec<UsageVolumeProductRef>>,
    ///Date and time of the request creation
    #[serde(rename = "creationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<crate::DateTime>,
    ///Date when the status was last changed
    #[serde(rename = "lastUpdate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update: Option<crate::DateTime>,
    ///Reference of a product
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<NetworkProductRef>,
    ///Reference and role of the related parties for which the usage consumption report is requested
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    ///Status of the usage consumption report request (InProgress or done)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///Reference of an usage consumption report
    #[serde(rename = "usageConsumptionReport")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_consumption_report: Option<UsageConsumptionReportRef>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validPeriod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_period: Option<TimePeriod>,
}
impl std::fmt::Display for UsageConsumptionReportRequestCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
