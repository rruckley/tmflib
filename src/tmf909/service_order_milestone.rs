use serde::{Serialize, Deserialize};
use super::ServiceOrderItemRef;
///ServiceOrderMilestone represents an action or event marking a significant change or stage in processing of a service order.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceOrderMilestone {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///A URI to a JSON-Schema file that defines additional attributes and relationships
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///When sub-classing, this defines the sub-class Extensible name
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///free-text description of the Milestone
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///identifier of the Milestone
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///A string represents the message of the milestone
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    ///A date time( DateTime). The date that the milestone happens
    #[serde(rename = "milestoneDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone_date: Option<crate::DateTime>,
    ///A string used to give a name to the milestone
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///A list of order item references corresponded to this milestone
    #[serde(rename = "serviceOrderItem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_order_item: Option<Vec<ServiceOrderItemRef>>,
    ///The milestone status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for ServiceOrderMilestone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
