use serde::{Serialize, Deserialize};
use super::TimePeriod;
///The period of time for which Capacity or CapacityDemand applies.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicableTimePeriod {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///A URI to a JSON-Schema file that defines additional attributes and relationships
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///When sub-classing, this defines the sub-class Extensible name
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///A day or days representing when the schedule is applicable. For example 2, 3 represent Monday and Tuesday.
    #[serde(rename = "dayOfWeek")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "fromToDateTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_to_date_time: Option<TimePeriod>,
    /**An indicator that specifies the inclusion or exclusion of the from and to DateTime attributes. 
Possible values are "open", "closed", "closedBottom" and "closedTop".*/
    #[serde(rename = "rangeInterval")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_interval: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for ApplicableTimePeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
