use crate::common::extensible::Extensible;
use serde::{Deserialize, Serialize};

///Monitored Class Criteria
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoredClassCriteria {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///A monitored object class for specifying the set of instances that are referenced by a PM query.
    #[serde(rename = "monitoredObjectClass")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitored_object_class: Option<String>,
    ///A filter that can be used in conjunction with the monitored object class for specifying the set of instances that are referenced by a PM query.
    #[serde(rename = "objectInstanceFilter")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_instance_filter: Option<String>,
}
impl std::fmt::Display for MonitoredClassCriteria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for MonitoredClassCriteria {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for MonitoredClassCriteria {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
