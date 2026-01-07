use serde::{Serialize, Deserialize};
use super::UsageConsumptionReportRequest;
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageConsumptionReportRequestStateChangeEventPayload {
    ///An UsageConsumptionReportRequest allows to manage the calculation request of an usage consumption report
    #[serde(rename = "usageConsumptionReportRequest")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_consumption_report_request: Option<UsageConsumptionReportRequest>,
}
impl std::fmt::Display for UsageConsumptionReportRequestStateChangeEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
