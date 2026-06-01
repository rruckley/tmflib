use super::EntityRefMvo;
use crate::common::extensible::Extensible;
use serde::{Deserialize, Serialize};

///`ServiceSpecificationRefMvo` represents a reference to a service specification, which may include the version of the service specification. It is used to define the reference to a service specification in a specific context, e.g. for a specific customer or in a specific environment.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceSpecificationRefMvo {
    ///Base entity schema for use in `TMForum` Open-APIs. Property.
    #[serde(flatten)]
    pub entity_ref_mvo: EntityRefMvo,
    ///Base Extensible schema for use in `TMForum` Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Service specification version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for ServiceSpecificationRefMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ServiceSpecificationRefMvo {
    type Target = EntityRefMvo;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref_mvo
    }
}
impl std::ops::DerefMut for ServiceSpecificationRefMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref_mvo
    }
}
