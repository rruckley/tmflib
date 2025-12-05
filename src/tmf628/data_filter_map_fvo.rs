use super::DataFilterMapItemFvo;
use crate::common::extensible::ExtensibleFvo;
use serde::{Deserialize, Serialize};

///Data Filter Map Full Value Object``
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataFilterMapFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    ///Mappings between filter names and values
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mappings: Vec<DataFilterMapItemFvo>,
}
impl std::fmt::Display for DataFilterMapFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for DataFilterMapFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for DataFilterMapFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
