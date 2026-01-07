use serde::{Serialize, Deserialize};
use super::ServiceRefOrValue;
///Alternate service proposal is used when the requested service is not available with characteristic and date asked for. An alternate proposal could be a distinct serviceSpecification close to requested one or same as requested but with a different activation date
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternateServiceProposal {
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
    ///A Service to be created defined by value or existing defined by reference. The polymorphic attributes @type, @schemaLocation & @referredType are related to the Service entity and not the RelatedServiceRefOrValue class itself
    #[serde(rename = "alternateService")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alternate_service: Option<ServiceRefOrValue>,
    ///Alternate availability date in case seller is not able to meet requested expected availability date for the service
    #[serde(rename = "alternateServiceAvailabilityDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alternate_service_availability_date: Option<crate::DateTime>,
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///Identifier of a alternate service proposal
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl std::fmt::Display for AlternateServiceProposal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
