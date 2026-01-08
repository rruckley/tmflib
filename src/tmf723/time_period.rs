use serde::{Serialize, Deserialize};
///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimePeriodYYY {
    ///End of the time period, using IETC-RFC-3339 format
    #[serde(rename = "endDateTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<crate::DateTime>,
    ///Start of the time period, using IETC-RFC-3339 format
    #[serde(rename = "startDateTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<crate::DateTime>,
}
impl std::fmt::Display for TimePeriodYYY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
