use serde::{Serialize, Deserialize};
use super::{ServiceCategoryRef, ServiceRefOrValue};
///A ServiceQualificationItem relates to a specific service being checked in a qualification operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceQualificationItem {
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
    ///The (service) category resource is used to group service candidates in logical containers. Categories can contain other categories.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<ServiceCategoryRef>,
    ///The date when the service is expected to be activated
    #[serde(rename = "expectedActivationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_activation_date: Option<crate::DateTime>,
    ///Date when the requester looks for service availability
    #[serde(rename = "expectedServiceAvailabilityDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_service_availability_date: Option<crate::DateTime>,
    ///Date when the qualification item response expires
    #[serde(rename = "expirationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<crate::DateTime>,
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///Id of the Service Qualification Item
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///A Service to be created defined by value or existing defined by reference. The polymorphic attributes @type, @schemaLocation & @referredType are related to the Service entity and not the RelatedServiceRefOrValue class itself
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceRefOrValue>,
}
impl std::fmt::Display for ServiceQualificationItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
