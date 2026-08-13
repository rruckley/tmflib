use serde::{Serialize, Deserialize};
use super::{
    AttachmentRefOrValue, ChangeRequestRelationship, ChangeRequestStatusType,
    Characteristic, EntitySpecificationRef, ExternalReference, ImpactEntity, Money, Note,
    RelatedEntity, RelatedParty, RelatedPlaceRefOrValue, Resolution, ServiceProblemRef,
    SlaRef, TroubleTicketRef, WorkLog,
};

use crate::DateTime;

///A Service to be created defined by value or existing defined by reference. The polymorphic attributes @type, @schemaLocation & @referredType are related to the Service entity and not the RelatedServiceRefOrValue class itself
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeRequestRefOrValue {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///The actual type of the target instance when needed for disambiguation.
    #[serde(rename = "@referredType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referred_type: Option<String>,
    ///A URI to a JSON-Schema file that defines additional attributes and relationships
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///When sub-classing, this defines the sub-class Extensible name
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///Date and time when the change implementation actually finished
    #[serde(rename = "actualEndTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_end_time: Option<DateTime>,
    ///Date and time when the change implementation actually started
    #[serde(rename = "actualStartTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_start_time: Option<DateTime>,
    ///The attachments of the communication message (when it is email type)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<AttachmentRefOrValue>>,
    ///A base / value business entity used to represent money
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub budget: Option<Money>,
    #[serde(rename = "changeRelationship")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_relationship: Option<Vec<ChangeRequestRelationship>>,
    #[serde(rename = "changeRequestCharacteristic")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_request_characteristic: Option<Vec<Characteristic>>,
    ///A channel represents the way the Change Request was created
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    ///Date and time when the change request is confirmed to be completed
    #[serde(rename = "completionDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<DateTime>,
    ///Description of the change request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalReference")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<Vec<ExternalReference>>,
    ///Hyperlink to access a change request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///Identifier of a Change Request. It is created on repository side (a Change Management system)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Indicates the impact of this change
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact: Option<String>,
    #[serde(rename = "impactEntity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_entity: Option<Vec<ImpactEntity>>,
    ///Date and time when the change request was updated
    #[serde(rename = "lastUpdateDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update_date: Option<DateTime>,
    ///Related Entity reference. A related place defines a place described by reference or by value linked to a specific entity. The polymorphic attributes @type, @schemaLocation & @referredType are related to the place entity and not the RelatedPlaceRefOrValue class itself
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<RelatedPlaceRefOrValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Note>>,
    ///Date and time when the change implementation is planned to be finished
    #[serde(rename = "plannedEndTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planned_end_time: Option<DateTime>,
    ///Date and time when the change implementation is planned to be started
    #[serde(rename = "plannedStartTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planned_start_time: Option<DateTime>,
    ///Used by consumers to prioritize a change request in Change Management system
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(rename = "problemTicket")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub problem_ticket: Option<Vec<ServiceProblemRef>>,
    ///The parties involved in the change request
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    ///Date and time when the change request is raised
    #[serde(rename = "requestDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_date: Option<DateTime>,
    ///Indicates the type of the change request
    #[serde(rename = "requestType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_type: Option<String>,
    ///The way one or more change request has been implementation through a direct remedy or task
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Resolution>,
    ///The risk to implement this change request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk: Option<String>,
    ///The risk mitigation plan
    #[serde(rename = "riskMitigationPlan")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_mitigation_plan: Option<String>,
    ///The additional cost if the risk will happen
    #[serde(rename = "riskValue")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_value: Option<String>,
    ///Date and time that the schedule is made
    #[serde(rename = "scheduledDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduled_date: Option<DateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sla: Option<Vec<SlaRef>>,
    ///reference to an EntitySpecification object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specification: Option<EntitySpecificationRef>,
    ///Possible values for the state of the change request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ChangeRequestStatusType>,
    ///Date and time when the change request status was changed
    #[serde(rename = "statusChangeDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_change_date: Option<DateTime>,
    ///Reason of the status change
    #[serde(rename = "statusChangeReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_change_reason: Option<String>,
    #[serde(rename = "targetEntity")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_entity: Vec<RelatedEntity>,
    #[serde(rename = "troubleTicket")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trouble_ticket: Option<Vec<TroubleTicketRef>>,
    #[serde(rename = "workLog")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_log: Option<Vec<WorkLog>>,
}
impl std::fmt::Display for ChangeRequestRefOrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
