use serde::{Serialize, Deserialize};
use super::ServiceOrderItemRef;
///A ServiceOrderJeopardyAlert represents a predicted exception during a service order processing that would brings risk to complete successfully the ordetr.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceOrderJeopardyAlert {
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
    ///A date time( DateTime). The date that the alert issued
    #[serde(rename = "alertDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alert_date: Option<crate::DateTime>,
    ///The exception associated with this jeopardy alert
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exception: Option<String>,
    ///identifier of the JeopardyAlert
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///A string represents the type of jeopardy/risk like Normal, Hazard, Critical, ...
    #[serde(rename = "jeopardyType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jeopardy_type: Option<String>,
    ///A string represents the message of the alert
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    ///A string used to give a name to the jeopardy alert
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///A list of order item references corresponded to this alert
    #[serde(rename = "serviceOrderItem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_order_item: Option<Vec<ServiceOrderItemRef>>,
}
impl std::fmt::Display for ServiceOrderJeopardyAlert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
