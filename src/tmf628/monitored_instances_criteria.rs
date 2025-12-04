use crate::common::extensible::Extensible;
use serde::{Deserialize, Serialize};

/// Monitored Instances Criteria
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoredInstancesCriteria {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    /// List of monitored object instances
    #[serde(rename = "monitoredObjectInstances")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monitored_object_instances: Vec<String>,
}
impl std::fmt::Display for MonitoredInstancesCriteria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for MonitoredInstancesCriteria {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for MonitoredInstancesCriteria {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
