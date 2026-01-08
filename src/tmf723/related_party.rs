use serde::{Serialize, Deserialize};
use super::{EntityRef, Extensible, Reference};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelatedParty {
    ///Entity reference schema to be use for all entityRef class.
    #[serde(flatten)]
    pub entity_ref: EntityRef,
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Reference schema .
    #[serde(flatten)]
    pub reference: Reference,
    ///Role played by the related party
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
impl std::fmt::Display for RelatedParty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for RelatedParty {
    type Target = EntityRef;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref
    }
}
impl std::ops::DerefMut for RelatedParty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref
    }
}
