use super::Extensible;
use serde::{Deserialize, Serialize};

/// Authorization related to an agreement
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgreementAuthorization {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///The date associated with the authorization state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::DateTime>,
    ///Indication that represents whether the signature is a physical paper signature or a digital signature.
    #[serde(rename = "signatureRepresentation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_representation: Option<String>,
    ///Current status of the authorization, for example in process, approved, rejected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl std::fmt::Display for AgreementAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AgreementAuthorization {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for AgreementAuthorization {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
