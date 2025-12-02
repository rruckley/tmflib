use serde::{Serialize, Deserialize};
use super::{AgreementSpecificationRef, PartyRoleSpecificationRef};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartyPrivacyRoleSpecification {
    ///Party role specification reference. A party role specification gives additional details on the part played by a party in a given context.
    #[serde(flatten)]
    pub party_role_specification_ref: PartyRoleSpecificationRef,
    #[serde(rename = "agreementSpecification")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agreement_specification: Vec<AgreementSpecificationRef>,
}
impl std::fmt::Display for PartyPrivacyRoleSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PartyPrivacyRoleSpecification {
    type Target = PartyRoleSpecificationRef;
    fn deref(&self) -> &Self::Target {
        &self.party_role_specification_ref
    }
}
impl std::ops::DerefMut for PartyPrivacyRoleSpecification {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.party_role_specification_ref
    }
}
