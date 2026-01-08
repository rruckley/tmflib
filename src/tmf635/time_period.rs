use serde::{Deserialize, Serialize};
///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimePeriodXX {
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
    ///End of the time period, using IETC-RFC-3339 format
    #[serde(rename = "endDateTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<crate::DateTime>,
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///unique identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Start of the time period, using IETC-RFC-3339 format
    #[serde(rename = "startDateTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<crate::DateTime>,
}
impl std::fmt::Display for TimePeriodXX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
