use serde::{Serialize, Deserialize};
use super::{EntityRef, RelatedPartyRefOrPartyRoleRef};
use crate::common::extensible::Extensible;

///An update of the context of an entity, such as status or state
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContextUpdate {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Identifier of the context update
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///The date and time the status/state last changed.
    #[serde(rename = "lastUpdate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update: Option<crate::DateTime>,
    ///The reason/context for the current value of the status/state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    ///A list of entities that represent observations, anomalies or managing entities etc associated to the current status/state value.
    #[serde(rename = "relatedEntity")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_entity: Vec<EntityRef>,
    ///List of parties associated to the current status/state value.
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_party: Vec<RelatedPartyRefOrPartyRoleRef>,
}
impl std::fmt::Display for ContextUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ContextUpdate {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for ContextUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
