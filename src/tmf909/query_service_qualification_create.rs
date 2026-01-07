use serde::{Serialize, Deserialize};
use super::{RelatedParty, ServiceQualificationItem};
/**QueryServiceQualification is used to retrieve a list of services that are technically available in the context of the interaction (place, party, service characteristics, ...).
Skipped properties: id,href,serviceQualificationItem,state,effectiveQualificationDate,expirationDate,estimatedResponseDate,queryServiceQualificationDate*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryServiceQualificationCreate {
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
    ///Description of the serviceQualification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///A date (DateTime). Deadline date when the requester expected a qualification answer.
    #[serde(rename = "expectedQualificationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_qualification_date: Option<crate::DateTime>,
    ///Identifier provided by the requester
    #[serde(rename = "externalId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    ///An indicator which when the value is "true" means that requester expects to get qualifcation result immediately in the response. If the indicator is true then the response code of 200 indicates the operation is successful otherwise a task is created with a response 201.
    #[serde(rename = "instantSyncQualification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instant_sync_qualification: Option<bool>,
    ///A list of related party references, defines party or party role linked to this request.
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    ///A ServiceQualificationItem relates to a specific service being checked in a qualification operation.
    #[serde(rename = "searchCriteria")]
    pub search_criteria: ServiceQualificationItem,
}
impl std::fmt::Display for QueryServiceQualificationCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
