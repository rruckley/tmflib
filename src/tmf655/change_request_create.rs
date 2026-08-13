use serde::{Serialize, Deserialize};
use super::{
    AttachmentRefOrValue, ChangeRequestRelationship, Characteristic,
    EntitySpecificationRef, ExternalReference, ImpactEntity, Money, Note, RelatedEntity,
    RelatedParty, RelatedPlaceRefOrValue, Resolution, ServiceProblemRef, SlaRef,
    TroubleTicketRef, WorkLog,
};
/**Change Request is a type of request which can be used for the management and control of Change Management process
 -within a service provider organisation or
 -between a customer and a service provider or
 -between a service provider and a partner and vice versa.
Skipped properties: id,href,lastUpdateTime,statusChangeDate,statusChangeReason,status,actualEndTime,actualStartTime,completionDate,lastUpdateDate*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChangeRequestCreate {
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
    ///Description of the change request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalReference")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<Vec<ExternalReference>>,
    ///Indicates the impact of this change
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact: Option<String>,
    #[serde(rename = "impactEntity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_entity: Option<Vec<ImpactEntity>>,
    ///Related Entity reference. A related place defines a place described by reference or by value linked to a specific entity. The polymorphic attributes @type, @schemaLocation & @referredType are related to the place entity and not the RelatedPlaceRefOrValue class itself
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<RelatedPlaceRefOrValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Note>>,
    ///Date and time when the change implementation is planned to be finished
    #[serde(rename = "plannedEndTime")]
    pub planned_end_time: chrono::DateTime<chrono::Utc>,
    ///Date and time when the change implementation is planned to be started
    #[serde(rename = "plannedStartTime")]
    pub planned_start_time: chrono::DateTime<chrono::Utc>,
    ///Used by consumers to prioritize a change request in Change Management system
    pub priority: String,
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
    pub request_date: Option<chrono::DateTime<chrono::Utc>>,
    ///Indicates the type of the change request
    #[serde(rename = "requestType")]
    pub request_type: String,
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
    pub scheduled_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sla: Option<Vec<SlaRef>>,
    ///reference to an EntitySpecification object
    pub specification: EntitySpecificationRef,
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
impl std::fmt::Display for ChangeRequestCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
