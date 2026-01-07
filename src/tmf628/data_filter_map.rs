use super::DataFilterMapItem;
use crate::common::extensible::Extensible;
use serde::{Deserialize, Serialize};

///Data Filter Map
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataFilterMap {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///A list of data filter map items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mappings: Vec<DataFilterMapItem>,
}
impl std::fmt::Display for DataFilterMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for DataFilterMap {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for DataFilterMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
