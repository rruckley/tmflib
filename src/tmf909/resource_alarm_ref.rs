use serde::{Serialize, Deserialize};
use super::EntityRef;
///A set of alarm ids identifying the alarms that are underlying this problem.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceAlarmRef {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///The actual type of the target instance when needed for disambiguation.
    #[serde(rename = "@referredType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referred_type: Option<String>,
    ///A URI to a JSON-Schema file that defines additional attributes and relationships
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///When sub-classing, this defines the sub-class Extensible name
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///Entity reference schema to be use for all entityRef class.
    #[serde(rename = "changeRequest")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_request: Option<EntityRef>,
    ///Reference of the Alarm
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///Unique identifier of the Alarm
    pub id: String,
}
impl std::fmt::Display for ResourceAlarmRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
