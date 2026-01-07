use serde::{Serialize, Deserialize};
use super::{
    AlternateServiceProposal, ServiceCategoryRef, ServiceEligibilityUnavailabilityReason,
    ServiceQualificationItemRelationship, ServiceQualificationRelationship,
    ServiceRefOrValue, TerminationError,
};
///A ServiceQualificationItem relates to a specific service being checked in a qualification operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckServiceQualificationItem {
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
    ///Alternate service proposal is used when the requested service is not available with characteristic and date asked for. An alternate proposal could be a distinct service Spec close to requested one or same as requested but with a different availability date.
    #[serde(rename = "alternateServiceProposal")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alternate_service_proposal: Option<Vec<AlternateServiceProposal>>,
    ///The (service) category resource is used to group service candidates in logical containers. Categories can contain other categories.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<ServiceCategoryRef>,
    ///A list of eligibility unavailability reasons (EligibilityUnavailabilityReason [*]). Reason for eligibility result if the serviceQualification result is no (meaning the service is not available).
    #[serde(rename = "eligibilityUnavailabilityReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eligibility_unavailability_reason: Option<
        Vec<ServiceEligibilityUnavailabilityReason>,
    >,
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
    ///unique identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///A list of qualification item relationships used to describe relationship between serviceQualification item from the same serviceQualification.
    #[serde(rename = "qualificationItemRelationship")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualification_item_relationship: Option<
        Vec<ServiceQualificationItemRelationship>,
    >,
    ///Structure used to describe relationship between serviceQualification item from the same serviceQualification.
    #[serde(rename = "qualificationRelationship")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualification_relationship: Option<Vec<ServiceQualificationRelationship>>,
    ///Qualification result for serviceQualification item. It could be: - qualified (request service are available), - unqualified (requested not available and not alternate available), - alternate (requested not available but proposal available)
    #[serde(rename = "qualificationResult")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualification_result: Option<String>,
    ///A Service to be created defined by value or existing defined by reference. The polymorphic attributes @type, @schemaLocation & @referredType are related to the Service entity and not the RelatedServiceRefOrValue class itself
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceRefOrValue>,
    ///State of the serviceQualification item (acknowledged, inProgress, terminatedWithError, done)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    ///If qualificationItem has not been done properly this lists the error(s) that caused termination of the qualification.
    #[serde(rename = "terminationError")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_error: Option<Vec<TerminationError>>,
}
impl std::fmt::Display for CheckServiceQualificationItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
