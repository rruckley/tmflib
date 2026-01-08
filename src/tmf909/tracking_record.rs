use serde::{Serialize, Deserialize};
use super::Characteristic;
///Tracking records allow the tracking of modifications on the problem. The tracking records should not be embedded in the problem to allow retrieving the problem without the tracking records
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrackingRecord {
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
    ///A generic list of any type of elements. Used for vendor Extensions or loose element encapsulation from other namespaces
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub characteristic: Option<Vec<Characteristic>>,
    ///Describes the action being done, such as: ack, clear
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Identifier of the TrackingRecord
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Describes the system Id from which the action was done
    #[serde(rename = "systemId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_id: Option<String>,
    ///Describes the time at which the action was done
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<crate::DateTime>,
    ///Describes the user doing the action
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
impl std::fmt::Display for TrackingRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
