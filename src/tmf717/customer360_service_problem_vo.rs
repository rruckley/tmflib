use serde::{Serialize, Deserialize};
use super::{Entity, ServiceRef};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Customer360ServiceProblemVo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///List of affected services. At least one of affectedResource, affectedService or affectedLocation should be present.
    #[serde(rename = "affectedService")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub affected_service: Vec<ServiceRef>,
    ///Classifier for the problem. Settable. For example, this is used for distinguish the category of problem originator in [role].[category] format. Example: serviceProvider.declarer, supplier.originated, system.originated
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    ///Free form text describing the Service Problem
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Indicates where the problem was generated
    #[serde(rename = "originatingSystem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originating_system: Option<String>,
    ///An indication varying from 1 (highest) to 10 (lowest) of how important it is for the service provider to correct the Service Problem.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    ///The current status of the service problem. Possible values are Submitted, Rejected, Acknowledged, In Progress [Held, Pending], Resolved, Closed, and Cancelled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for Customer360ServiceProblemVo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Customer360ServiceProblemVo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Customer360ServiceProblemVo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
