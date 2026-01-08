use serde::{Serialize, Deserialize};
use super::CheckServiceQualification;
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckServiceQualificationCreateEventPayload {
    ///CheckServiceQualification is used to perform a technical eligibility on service configuration(s). It allows to retrieve services that are technically available in the context of the interaction (place, party, service characteristics, ...).
    #[serde(rename = "checkServiceQualification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_service_qualification: Option<CheckServiceQualification>,
}
impl std::fmt::Display for CheckServiceQualificationCreateEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
