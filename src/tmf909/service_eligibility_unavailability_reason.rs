use serde::{Serialize, Deserialize};
///Reason for eligibility result if the ServiceQualification result is no (meaning the Service is not available)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceEligibilityUnavailabilityReason {
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
    ///Unavailability reason code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///unique identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Unavailability reason label
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
impl std::fmt::Display for ServiceEligibilityUnavailabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
