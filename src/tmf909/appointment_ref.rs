use serde::{Serialize, Deserialize};
///Refers an appointment, such as a Customer presentation or internal meeting or site visit
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppointmentRef {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///The actual type of the target instance when needed for disambiguation
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
    ///An explanatory text regarding the appointment made with a party
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The reference of the appointment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///The identifier of the referred appointment
    pub id: String,
}
impl std::fmt::Display for AppointmentRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
