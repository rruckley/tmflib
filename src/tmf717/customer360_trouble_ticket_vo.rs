use serde::{Serialize, Deserialize};
use super::{Entity, TroubleTicketStatusType};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer360TroubleTicketVo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///The date on which the trouble ticket was created
    #[serde(rename = "creationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<crate::DateTime>,
    ///Description of the trouble or issue
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The expected resolution date determined by the trouble ticket system
    #[serde(rename = "expectedResolutionDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_resolution_date: Option<crate::DateTime>,
    ///Additional identifier coming from an external system
    #[serde(rename = "externalIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_identifier: Option<String>,
    ///The date and time that the trouble ticked was last updated
    #[serde(rename = "lastUpdate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update: Option<crate::DateTime>,
    ///Name of the trouble ticket, typically a short description provided by the user that create the ticket
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The priority of the trouble ticket and how quickly the issue should be resolved. Example: Critical, High, Medium, Low. The value is set by the ticket management system considering the severity, ticket type etc...
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    ///The resolution date requested by the user
    #[serde(rename = "requestedResolutionDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_resolution_date: Option<crate::DateTime>,
    /**The severity of the issue. Indicate the implication of the issue on the expected functionality e.g. of a system, application, service etc..
Severity values can be for example : Critical, Major, Minor*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    ///Possible values for the status of the trouble ticket
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TroubleTicketStatusType>,
    ///The date and time the status changed.
    #[serde(rename = "statusChangeDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_change_date: Option<crate::DateTime>,
    ///The reason for changing the status
    #[serde(rename = "statusChangeReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_change_reason: Option<String>,
    ///represent a business type of the trouble ticket e.g. incident, complain, request
    #[serde(rename = "ticketType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket_type: Option<String>,
}
impl std::fmt::Display for Customer360TroubleTicketVo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Customer360TroubleTicketVo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Customer360TroubleTicketVo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
