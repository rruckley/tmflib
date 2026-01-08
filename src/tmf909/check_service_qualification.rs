use serde::{Serialize, Deserialize};
use super::{CheckServiceQualificationItem, RelatedParty, TaskStateType};
///CheckServiceQualification is used to perform a technical eligibility on service configuration(s). It allows to retrieve services that are technically available in the context of the interaction (place, party, service characteristics, ...).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckServiceQualification {
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
    ///Date when the serviceQualification was submitted
    #[serde(rename = "checkServiceQualificationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_service_qualification_date: Option<crate::DateTime>,
    ///Description of the serviceQualification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Effective date to serviceQualification completion
    #[serde(rename = "effectiveQualificationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_qualification_date: Option<crate::DateTime>,
    ///Date when the requester expect to provide an answer for the qualification request.
    #[serde(rename = "estimatedResponseDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_response_date: Option<crate::DateTime>,
    ///A date (DateTime). Deadline date when the requester expected a qualification answer.
    #[serde(rename = "expectedQualificationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_qualification_date: Option<crate::DateTime>,
    ///Date when the qualification response expires
    #[serde(rename = "expirationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<crate::DateTime>,
    ///Identifier provided by the requester
    #[serde(rename = "externalId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///unique identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///An indicator which when the value is "true" means that requester expects to get qualifcation result immediately in the response. If the indicator is true then the response code of 200 indicates the operation is successful otherwise a task is created with a response 201.
    #[serde(rename = "instantSyncQualification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instant_sync_qualification: Option<bool>,
    ///When the value is TRUE means that alternative solutions should be provided
    #[serde(rename = "provideAlternative")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provide_alternative: Option<bool>,
    ///When the value is TRUE means that unavailability reason are expected for non available service.
    #[serde(rename = "provideUnavailabilityReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provide_unavailability_reason: Option<bool>,
    ///Qualification result for this serviceQualification. It could be:  qualified (all qualification item are qualified), alternate (At least one item alternate and no item with  unqualified), unqualified (At least one item unqualified)
    #[serde(rename = "qualificationResult")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualification_result: Option<String>,
    ///A list of related party references, defines party or party role linked to this request.
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    ///A list of service qualification items.
    #[serde(rename = "serviceQualificationItem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_qualification_item: Option<Vec<CheckServiceQualificationItem>>,
    ///Possible values for the state of a task
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<TaskStateType>,
}
impl std::fmt::Display for CheckServiceQualification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
