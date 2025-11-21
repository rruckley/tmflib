use serde::{Serialize, Deserialize};
use crate::common::extensible::Extensible;

///Data Filter Template FVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataFilterTemplate {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Description of this DataFilterTemplate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Hyperlink reference to this DataFilterTemplate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///Unique identifier of this DataFilterTemplate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Name given to this DataFilterTemplate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for DataFilterTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for DataFilterTemplate {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for DataFilterTemplate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
