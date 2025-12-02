use serde::{Serialize, Deserialize};
use super::{AgreementSpecificationRefMvo, PartyRoleSpecificationRefMvo};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyPrivacyRoleSpecificationMvo {
    ///Party role specification reference. A party role specification gives additional details on the part played by a party in a given context.
    #[serde(flatten)]
    pub party_role_specification_ref_mvo: PartyRoleSpecificationRefMvo,
    #[serde(rename = "agreementSpecification")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agreement_specification: Vec<AgreementSpecificationRefMvo>,
}
impl std::fmt::Display for PartyPrivacyRoleSpecificationMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PartyPrivacyRoleSpecificationMvo {
    type Target = PartyRoleSpecificationRefMvo;
    fn deref(&self) -> &Self::Target {
        &self.party_role_specification_ref_mvo
    }
}
impl std::ops::DerefMut for PartyPrivacyRoleSpecificationMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.party_role_specification_ref_mvo
    }
}
