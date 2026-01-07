use serde::{Serialize, Deserialize};
use super::QueryServiceQualification;
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryServiceQualificationCreateEventPayload {
    ///QueryServiceQualification is used to retrieve a list of services that are technically available in the context of the interaction (place, party, service characteristics, ...).
    #[serde(rename = "queryServiceQualification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_service_qualification: Option<QueryServiceQualification>,
}
impl std::fmt::Display for QueryServiceQualificationCreateEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
