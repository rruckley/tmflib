use serde::{Deserialize, Serialize};

use crate::common::entity_ref::EntityRef;
use crate::common::extensible::Extensible;

/// Reference to an AgreementSpecification entity
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgreementSpecificationRef {
    ///Base entity reference schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity_ref: EntityRef,
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///A narrative that explains in detail what the agreement specification is about.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Name of the agreement specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for AgreementSpecificationRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AgreementSpecificationRef {
    type Target = EntityRef;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref
    }
}
impl std::ops::DerefMut for AgreementSpecificationRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref
    }
}
