use serde::{Serialize, Deserialize};
use super::ExtensibleFvo;

/// Monitored Instances Criteria Fvo
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoredInstancesCriteriaFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    /// List of monitored object instances
    #[serde(rename = "monitoredObjectInstances")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monitored_object_instances: Vec<String>,
}
impl std::fmt::Display for MonitoredInstancesCriteriaFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for MonitoredInstancesCriteriaFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for MonitoredInstancesCriteriaFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
