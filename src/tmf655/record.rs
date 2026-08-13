use serde::{Serialize, Deserialize};
///A record in a worklog.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Record {
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
    ///The date time that a record is generated.
    #[serde(rename = "dateTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_time: Option<chrono::DateTime<chrono::Utc>>,
    ///The detail description in a record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///unique identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///The person who logged that record.
    #[serde(rename = "supportPerson")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_person: Option<String>,
}
impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
