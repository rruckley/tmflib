use super::{AgreementSpecificationRefFvo, PartyRoleSpecificationRefFvo};
use serde::{Deserialize, Serialize};

/// Party Privacy Role Specification FVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyPrivacyRoleSpecificationFvo {
    ///Party role specification reference. A party role specification gives additional details on the part played by a party in a given context.
    #[serde(flatten)]
    pub party_role_specification_ref_fvo: PartyRoleSpecificationRefFvo,
    ///Agreement specifications associated with this party role specification.
    #[serde(rename = "agreementSpecification")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agreement_specification: Vec<AgreementSpecificationRefFvo>,
}
impl std::fmt::Display for PartyPrivacyRoleSpecificationFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PartyPrivacyRoleSpecificationFvo {
    type Target = PartyRoleSpecificationRefFvo;
    fn deref(&self) -> &Self::Target {
        &self.party_role_specification_ref_fvo
    }
}
impl std::ops::DerefMut for PartyPrivacyRoleSpecificationFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.party_role_specification_ref_fvo
    }
}
