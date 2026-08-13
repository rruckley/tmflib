use serde::{Serialize, Deserialize};
use super::Record;
///A record of the work performed on the change request during the investigation and resolution process.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkLog {
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
    ///Date and time of worklog generated.
    #[serde(rename = "createDateTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_date_time: Option<chrono::DateTime<chrono::Utc>>,
    ///The description of the worklog.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///unique identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Date and time when the worklog updated.
    #[serde(rename = "lastUpdateDateTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update_date_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<Vec<Record>>,
}
impl std::fmt::Display for WorkLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
