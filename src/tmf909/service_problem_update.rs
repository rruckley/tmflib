use serde::{Serialize, Deserialize};
use super::{
    Characteristic, EventRef, ExternalIdentifier, ImpactPattern, Note, RelatedEntity,
    RelatedParty, RelatedPlace, ResourceAlarmRef, ResourceRef, ServiceProblemRef,
    ServiceProblemStateType, ServiceRef, SlaViolationRef, TroubleTicketRef,
};
/**The problem information for Middle B which is abstracted in the service layer from the issued event information by First B
Skipped properties: id,href,firstAlert,originatingSystem,trackingRecord*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceProblemUpdate {
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
    ///Number of affected services
    #[serde(rename = "affectedNumberOfServices")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affected_number_of_services: Option<i64>,
    ///A list of the resources affected by the problem. At least one of affectedResource, affectedService or affectedLocation should be present.
    #[serde(rename = "affectedResource")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affected_resource: Option<Vec<ResourceRef>>,
    ///List of affected services. At least one of affectedResource, affectedService or affectedLocation should be present.
    #[serde(rename = "affectedService")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affected_service: Option<Vec<ServiceRef>>,
    ///Classifier for the problem. Settable. For example, this is used for distinguish the category of problem originator in [role].[category] format. Example: serviceProvider.declarer, supplier.originated, system.originated
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    ///This is a list of characteristics associated with the Service Problem
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub characteristic: Option<Vec<Characteristic>>,
    ///Time the problem was created
    #[serde(rename = "creationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<crate::DateTime>,
    ///Free form text describing the Service Problem
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalIdentifier")]
    ///This is a list of external identifiers associated with the Service Problem
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_identifier: Option<Vec<ExternalIdentifier>>,
    ///Impact Importance is characterized by an Impact Importance Factor: overall importance of the impact of all the affected services, e.g. 0 (zero impact) to 100 (worst impact). The Impact Importance is a calculated field which is set by the OSS determining the impact.
    #[serde(rename = "impactImportanceFactor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_importance_factor: Option<String>,
    ///Define the patterns of impact (optional), such as other service characteristics- Used when defining impact through another pattern than the pre-defined attributes
    #[serde(rename = "impactPattern")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impact_pattern: Option<ImpactPattern>,
    ///Time the problem was last changed
    #[serde(rename = "lastUpdate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update: Option<crate::DateTime>,
    ///Name of the Service Problem
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///A list of comments or notes made on the problem
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Note>>,
    ///Related Party reference. A related party defines party or party role linked to a specific entity.
    #[serde(rename = "originatorParty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_party: Option<RelatedParty>,
    ///The parent problem to which this problem is attached.
    #[serde(rename = "parentProblem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_problem: Option<Vec<ServiceProblemRef>>,
    ///A list of the locations affected by the problem. At least one of affectedResource, affectedService or affectedLocation should be present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place: Option<Vec<RelatedPlace>>,
    ///An indication varying from 1 (highest) to 10 (lowest) of how important it is for the service provider to correct the Service Problem.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    ///Indicates if this service problem has been escalated or not. Possible values are 0 to 10. A value of zero means no escalation. The meanings of values 1-10 are to be determined by the user of the interface, but they show increasing levels of escalation.
    #[serde(rename = "problemEscalation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub problem_escalation: Option<String>,
    ///Free text or optionally structured text. It can be Unknown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    ///List of entities associated with this problem
    #[serde(rename = "relatedEntity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_entity: Option<Vec<RelatedEntity>>,
    ///List of events associated to this problem
    #[serde(rename = "relatedEvent")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_event: Option<Vec<EventRef>>,
    ///List of parties or party roles playing a role within the service problem
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    ///Time the problem was resolved
    #[serde(rename = "resolutionDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution_date: Option<crate::DateTime>,
    ///Related Party reference. A related party defines party or party role linked to a specific entity.
    #[serde(rename = "responsibleParty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsible_party: Option<RelatedParty>,
    ///Resource(s) that are associated to the underlying service problems that are the Root Cause of this one if any (used only if applicable).
    #[serde(rename = "rootCauseResource")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_cause_resource: Option<Vec<ResourceRef>>,
    ///Service(s) that are associated to the underlying service problems that are the Root Cause of this one if any (used only if applicable)
    #[serde(rename = "rootCauseService")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_cause_service: Option<Vec<ServiceRef>>,
    ///A List of SLA violations associated with this problem.
    #[serde(rename = "slaViolation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sla_violation: Option<Vec<SlaViolationRef>>,
    ///Possible values for the state of the ServiceProblem
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ServiceProblemStateType>,
    ///Time the problem was last status changed
    #[serde(rename = "statusChangeDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_change_date: Option<crate::DateTime>,
    ///The reason of state change
    #[serde(rename = "statusChangeReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_change_reason: Option<String>,
    ///A list of trouble tickets associated with this problem.
    #[serde(rename = "troubleTicket")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trouble_ticket: Option<Vec<TroubleTicketRef>>,
    ///A list of alarms underlying this problem.
    #[serde(rename = "underlyingAlarm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underlying_alarm: Option<Vec<ResourceAlarmRef>>,
    ///A list of underlying problems. Relevant only if this problem is derived from other problems.
    #[serde(rename = "underlyingProblem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underlying_problem: Option<Vec<ServiceProblemRef>>,
}
impl std::fmt::Display for ServiceProblemUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
