//! Check Service Qualification Module

use serde::{Deserialize,Serialize};

use crate::{
    common::related_party::RelatedParty, HasDescription, HasId, HasRelatedParty, Uri, LIB_PATH, DateTime, vec_insert
};

use crate::tmf633::service_category::ServiceCategoryRef;
use crate::tmf641::service_order_item::ServiceRefOrValue;

use tmflib_derive::{
    HasId,
    HasDescription,
    HasRelatedParty,
};

const CLASS_PATH : &str = "checkServiceQualification";
use super::{TaskStateType, MOD_PATH};

///  Reason for service unavailability
#[derive(Clone,Debug,Default, Deserialize,Serialize)]
pub struct ServiceEligibilityUnavailabilityReason {
    code : String,
    label : String,
}

/// Alternative service
#[derive(Clone,Debug,Default, Deserialize,Serialize)]
pub struct AlternateServiceProposal {
    /// Date when this service is available
    alternate_service_availability_date: Option<DateTime>,
    /// Unique identifier
    id : String,

    // Referenced objects
    /// Reference to alternative service
    alternate_service : Option<ServiceRefOrValue>,
}

impl From<ServiceRefOrValue> for AlternateServiceProposal {
    fn from(value: ServiceRefOrValue) -> Self {
        AlternateServiceProposal {
            alternate_service_availability_date : value.has_started.clone(),
            id : CheckServiceQualification::get_uuid(),
            alternate_service : Some(value),
        }
    }
}

/// Check Service Qualification
#[derive(Clone,Debug,Default,HasId,HasDescription,HasRelatedParty, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckServiceQualificationItem {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id : Option<String>,
    /// HTTP URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description : Option<String>,
    // Linked Objects
    /// Related Parties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party : Option<Vec<RelatedParty>>,
    /// Linked Service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service : Option<ServiceRefOrValue>,
    /// Category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category : Option<ServiceCategoryRef>,
    /// Unavailability Reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eligibility_unavailability_reason : Option<Vec<ServiceEligibilityUnavailabilityReason>>,
    /// Alternative Services
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_service_proposal : Option<Vec<AlternateServiceProposal>>,
}

impl CheckServiceQualificationItem {
    /// Add an alternative service proposal
    pub fn alternate(&mut self, service : ServiceRefOrValue) {
        vec_insert(&mut self.alternate_service_proposal,AlternateServiceProposal::from(service));
    }

    /// Add unavailability reason
    pub fn reason(&mut self, code : impl Into<String>, label : impl Into<String>) {
        vec_insert(&mut self.eligibility_unavailability_reason,ServiceEligibilityUnavailabilityReason{ code: code.into(), label: label.into()});
    }
}

/// Check Service Qualification 
#[derive(Clone,Debug,Default,HasId,HasDescription,HasRelatedParty,Deserialize,Serialize)]
pub struct CheckServiceQualification {
    /// Unique Id
    pub id : Option<String>,
    /// HTTP Uri
    pub href : Option<Uri>,
    /// Description
    pub description : Option<String>,
    /// SQ Status
    pub state : Option<TaskStateType>,
        // Referenced modules
    /// Service Qualification Items
    pub service_qualification_item : Option<Vec<CheckServiceQualificationItem>>,

    // Dates
    check_service_qualification_date : Option<DateTime>,
    effective_qualification_date : Option<DateTime>,
    estimated_response_date : Option<String>,
    estimated_qualification_date : Option<String>,
    expiration_date : Option<String>,

    // Flags
    /// Quick Qualification
    pub instant_sync_qualification : Option<bool>,
    /// Add Alternatives
    pub provide_alternative : Option<bool>,
    /// Provide failure reason
    pub provide_unavailability_reason : Option<bool>,

    /// Related Parties
    pub related_party : Option<Vec<RelatedParty>>,
}

impl CheckServiceQualification {
    /// Create a new SQ Check
    pub fn new(desc : impl Into<String>) -> CheckServiceQualification {
        CheckServiceQualification {
            ..CheckServiceQualification::create()
        }
        .description(desc)
        .state(TaskStateType::default())
    }

    /// Set the status
    pub fn state(mut self, state : TaskStateType) -> CheckServiceQualification {
        self.state = Some(state);
        self
    }

    /// Add item to SQ Check
    pub fn item(mut self, item : CheckServiceQualificationItem) -> CheckServiceQualification{
        vec_insert(&mut self.service_qualification_item,item);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SQ_DESC : &str = "SQ Description";

    #[test]
    fn test_sq_create() {
        let sq = CheckServiceQualification::new(SQ_DESC);

        assert_eq!(sq.description.is_some(),true);
        assert_eq!(sq.description.unwrap(),SQ_DESC.to_string());
    }

    #[test]
    fn test_sq_item() {
        let mut item = CheckServiceQualificationItem::default();
        item.reason("code", "label");
        let sq = CheckServiceQualification::new("Qualification")
            .item(item);

        assert_eq!(sq.service_qualification_item.is_some(),true);
        assert_eq!(sq.service_qualification_item.unwrap().len(),1);
    }

    #[test]
    fn test_sq_alternative() {
        let mut alternate = ServiceRefOrValue::default();
        alternate.description = Some("Alternate Service".to_string());
        let mut item = CheckServiceQualificationItem::default();
        item.reason("code", "label");
        item.alternate(alternate);

        assert_eq!(item.alternate_service_proposal.is_some(),true);
        assert_eq!(item.alternate_service_proposal.unwrap().len(),1);
    }

    #[test]
    fn test_sq_state() {
        let sq = CheckServiceQualification::new("Qualification")
            .state(TaskStateType::InProgress);

        assert_eq!(sq.state.is_some(),true);
        assert_eq!(sq.state.unwrap(),TaskStateType::InProgress);
    }
}