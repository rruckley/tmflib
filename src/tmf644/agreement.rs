use serde::{Serialize, Deserialize};
use super::{
    AgreementAuthorization, AgreementItem, AgreementRelationship,
    AgreementSpecificationRef, Characteristic, Entity, PartyRefOrPartyRoleRef,
    RelatedDocumentRefOrValue, RelatedPartyRefOrPartyRoleRef,
};
use crate::TimePeriod;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Agreement {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    #[serde(rename = "agreementAuthorization")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agreement_authorization: Vec<AgreementAuthorization>,
    #[serde(rename = "agreementItem")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agreement_item: Vec<AgreementItem>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "agreementPeriod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agreement_period: Option<TimePeriod>,
    #[serde(rename = "agreementRelationship")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agreement_relationship: Vec<AgreementRelationship>,
    #[serde(rename = "agreementSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agreement_specification: Option<AgreementSpecificationRef>,
    ///The type of the agreement. For example commercial
    #[serde(rename = "agreementType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agreement_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<Characteristic>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "completionDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<TimePeriod>,
    ///Narrative that explains the agreement and details about the it , such as why the agreement is taking place.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "engagedParty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub engaged_party: Vec<PartyRefOrPartyRoleRef>,
    ///Date at which the agreement was initialized
    #[serde(rename = "initialDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_date: Option<crate::DateTime>,
    ///A human-readable name for the agreement
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Documents related to the agreement, suc as signed off contract
    #[serde(rename = "relatedDocument")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_document: Vec<RelatedDocumentRefOrValue>,
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_party: Vec<RelatedPartyRefOrPartyRoleRef>,
    ///An overview and goals of the Agreement
    #[serde(rename = "statementOfIntent")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_of_intent: Option<String>,
    ///The current status of the agreement. Typical values are: in process, approved and rejected
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///A string identifying the version of the agreement
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for Agreement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Agreement {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Agreement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
