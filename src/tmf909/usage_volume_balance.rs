use serde::{Serialize, Deserialize};
use super::Quantity;
use crate::TimePeriod;

///The balance (called UsageVolumeBalance in the SID model) defines the remaining allowed product usage quantity in terms of volume, time, currency or events. It corresponds to the initial allowed usage quantity minus the usage consumed on the bucket.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageVolumeBalance {
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
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///unique identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///An amount in a given unit
    #[serde(rename = "remainingValue")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_value: Option<Quantity>,
    ///Remaining value in a formatted string for the bucket given in the balance unit (for example 1.9 Gb). This formatted string could be used for display needs for example
    #[serde(rename = "remainingValueName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_value_name: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for UsageVolumeBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
