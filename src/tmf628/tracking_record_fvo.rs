use super::{CharacteristicFvo};
use serde::{Deserialize, Serialize};
use crate::common::extensible::ExtensibleFvo;

///Tracking Record FVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrackingRecordFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    ///A generic list of any type of elements. Used for vendor Extensions or loose element encapsulation from other namespaces
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<CharacteristicFvo>,
    ///Describes the action being done, such as: ack, clear
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Describes the system Id from which the action was done
    #[serde(rename = "systemId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_id: Option<String>,
    ///Describes the time at which the action was done
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<crate::DateTime>,
    ///Describes the user doing the action
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
impl std::fmt::Display for TrackingRecordFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TrackingRecordFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for TrackingRecordFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
