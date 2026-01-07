use super::ThresholdRef;
use serde::{Deserialize, Serialize};
///Identifies the details of the threshold that has been crossed.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CrossedThresholdInformation {
    ///Indicates the threshold crossing direction: up or down.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    ///Indicates the granularity at which the indicator is evaluated for threshold crossing
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    ///Indicates the name of indicator which crossed the threshold.
    #[serde(rename = "indicatorName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_name: Option<String>,
    ///Indicates the unit of the measurement of the indicator corresponding to the threshold that has been crossed.
    #[serde(rename = "indicatorUnit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_unit: Option<String>,
    ///Indicates the value of the indicator which crossed the threshold.
    #[serde(rename = "observedValue")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_value: Option<String>,
    /// Indicates a reference to the threshold that has been crossed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<ThresholdRef>,
    ///Indicates further information on the threshold crossing alarm.
    #[serde(rename = "thresholdCrossingDescription")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold_crossing_description: Option<String>,
}
impl std::fmt::Display for CrossedThresholdInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
