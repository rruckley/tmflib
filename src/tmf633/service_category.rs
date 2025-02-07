//! Service Category Module


use serde::{Deserialize, Serialize};
use chrono::Utc;

use crate::{
    HasId, 
    HasLastUpdate, 
    HasName, 
    HasDescription,
    HasValidity,
    TimeStamp, 
    TimePeriod,
    TMFEvent,
    LIB_PATH,
    Uri,
    vec_insert,
};
use tmflib_derive::{HasId, HasLastUpdate, HasDescription, HasName, HasValidity};
use crate::common::event::{Event,EventPayload};

use super::{service_candidate::ServiceCandidateRef, MOD_PATH};
const CLASS_PATH : &str = "serviceCategory";
const CAT_STATUS_NEW : &str = "new";
const CAT_VERS_NEW : &str = "1.0";

/// Service Category Reference
/// # Description
/// Reference to another service category in the catalog
#[derive(Clone,Default,Debug,Deserialize, Serialize)]
pub struct ServiceCategoryRef {
    href: Uri,
    id: String,
    name: String,
    version: Option<String>,
}

impl From<ServiceCategory> for ServiceCategoryRef {
    fn from(value: ServiceCategory) -> Self {
        ServiceCategoryRef {
            href: value.get_href(),
            id: value.get_id(),
            name: value.get_name(),
            version: value.version.clone(),
        }
    }
}

/// Service Category Event Type
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub enum ServiceCategoryEventType {
    /// Category Created
    #[default]
    ServiceCategoryCreateEvent,
    /// Category Updated
    ServiceCategoryChangeEvent,
    /// Category Deleted
    ServiceCategoryDeleteEvent,
}

/// Service Category Container
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct ServiceCategoryEvent {
    /// Impacted Category
    category : ServiceCategory,
}

impl TMFEvent<ServiceCategoryEvent> for ServiceCategory {
    fn event(&self) -> ServiceCategoryEvent {
        ServiceCategoryEvent {
            category: self.clone(),
        }
    }    
}

impl EventPayload<ServiceCategoryEvent> for ServiceCategory {
    type Subject = ServiceCategory;
    type EventType = ServiceCategoryEventType;

    fn to_event(&self,event_type : Self::EventType) -> Event<ServiceCategoryEvent,Self::EventType> {
        let now = Utc::now();
        let desc = format!("{:?} for {} [{}]",event_type,self.get_name(),self.get_id());
        let event_time = chrono::DateTime::from_timestamp(now.timestamp(),0).unwrap();

        Event {
            id : self.id.clone(),
            href: self.href.clone(),
            description: Some(desc),
            title: self.name.clone(),
            domain: Some(ServiceCategory::get_class()),
            event_type,
            event_time: event_time.to_string(),
            event: self.event(),
            time_occurred: Some(event_time.to_string()),
            ..Default::default()
        }
    }
}

/// Service Category
/// # Desecription
/// Categorisation for services in a service catalog
#[derive(Clone, Debug, Default, Deserialize, HasId, HasName, HasDescription, HasLastUpdate, HasValidity, Serialize)]
pub struct ServiceCategory {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<Uri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_root: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_update: Option<TimeStamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
        // META
    /// Base Type this type is derived from if creating sub-classes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@baseType")]
    pub base_type : Option<String>,
    /// Schema Definition of the sub-class (if required)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@schemaLocation")]
    pub schema_location: Option<Uri>,
    /// Name for this Type when sub-classing
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    pub r#type : Option<String>,
    // References
    #[serde(skip_serializing_if = "Option::is_none")]
    category : Option<Vec<ServiceCategoryRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_candidate: Option<Vec<ServiceCandidateRef>>,
}

impl ServiceCategory {
    /// Create a new category instance
    pub fn new(name : impl Into<String>) -> ServiceCategory {
        ServiceCategory {
            name: Some(name.into()),
            lifecycle_status: Some(CAT_STATUS_NEW.into()),
            version : Some(CAT_VERS_NEW.into()),
            ..ServiceCategory::create_with_time()
        }
    }

    /// Add a child category to this category
    pub fn child_category(mut self, category: ServiceCategoryRef) -> ServiceCategory {
        vec_insert(&mut self.category, category);
        self
    }

    /// Add a ServiceCandidate to this category
    pub fn candidate(mut self, candidate: ServiceCandidateRef) -> ServiceCategory {
        vec_insert(&mut self.service_candidate,candidate);
        self
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::tmf633::service_candidate::ServiceCandidate;

    const CAT_NAME : &str = "CAT_NAME";
    const CHILD_CAT : &str = "CHILD_CAT";
    const CANDIDATE_NAME : &str = "CANDIDATE_NAME";

    #[test]
    fn test_servicecategory_create() {
        let category = ServiceCategory::new(CAT_NAME);

        assert_eq!(category.get_name(),CAT_NAME.to_string());
    }

    #[test]
    fn test_category_into_ref() {
        let category = ServiceCategory::new(CAT_NAME);

        let cat_ref : ServiceCategoryRef = category.into();

        // name in ref should match input name of cat
        assert_eq!(CAT_NAME.to_string(),cat_ref.name);
    }

    #[test]
    fn test_category_addchild() {
        let child_cat = ServiceCategory::new(CHILD_CAT);
        let parent_cat = ServiceCategory::new(CAT_NAME)
            .child_category(ServiceCategoryRef::from(child_cat));

        // Vec should have been created
        assert_eq!(parent_cat.category.is_some(),true);
        // Vec should only have a single entry
        assert_eq!(parent_cat.category.unwrap().len(),1);
    }

    #[test]
    fn test_category_addcandidate() {
        let mut candidate = ServiceCandidate::default();
        // Need to generate an id here eles the conversion to Ref will panic
        candidate.generate_id();
        candidate.set_name(CANDIDATE_NAME);

        let category = ServiceCategory::new(CAT_NAME)
            .candidate(ServiceCandidateRef::from(candidate));

        // Vec should have been created
        assert_eq!(category.service_candidate.is_some(),true);
        // Vec should only have a single entry
        assert_eq!(category.service_candidate.unwrap().len(),1);
    }
}