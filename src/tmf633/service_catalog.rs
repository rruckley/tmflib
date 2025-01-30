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
    LIB_PATH,
    TMFEvent,
    Uri,
    vec_insert,
};
use crate::common::related_party::RelatedParty;
use crate::common::event::{Event,EventPayload};
use super::service_category::ServiceCategoryRef;
use tmflib_derive::{HasId, HasLastUpdate, HasDescription, HasName, HasValidity};

use super::MOD_PATH;
const CLASS_PATH : &str = "serviceCatalog";
const CAT_STATUS_NEW : &str = "new";
const CAT_VERS_NEW : &str = "1.0";

/// Service Catalog Event Type
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub enum ServiceCatalogEventType {
    /// Catalog Created
    #[default]
    ServiceCatalogCreateEvent,
    /// Catalog Updated
    ServiceCatalogChangeEvent,
    /// Catalog Batch Completed
    ServiceCatalogBatchEvent,
    /// Catalog Deteled
    ServiceCatalogDeleteEvent,
}

/// Service Catalot Event Container
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct ServiceCatalogEvent {
    /// Impacted Service Catalog
    catalog : ServiceCatalog,
}

impl TMFEvent<ServiceCatalogEvent> for ServiceCatalog {
    fn event(&self) -> ServiceCatalogEvent {
        ServiceCatalogEvent {
            catalog : self.clone()
        }
    }
}

impl EventPayload<ServiceCatalogEvent> for ServiceCatalog {
    type Subject = ServiceCatalog;
    type EventType = ServiceCatalogEvent;

    fn to_event(&self,event_type : Self::EventType) -> Event<ServiceCatalogEvent,Self::EventType> {
        let now = Utc::now();
        let desc = format!("{:?} for {} [{}]",event_type,self.get_name(),self.get_id());
        let event_time = chrono::DateTime::from_timestamp(now.timestamp(),0).unwrap();

        Event {
            domain: Some(ServiceCatalog::get_class()),
            description: Some(desc),
            event_type,
            event_time: event_time.to_string(),
            event: self.event(),
            id : self.id.clone(),
            href: self.href.clone(),
            // More efficient to clone the option ? 
            title: self.name.clone(),
            time_occurred: Some(event_time.to_string()),
            ..Default::default()
        }
    }
}

/// Service Catalog
#[derive(Clone, Debug, Default, Deserialize, HasId, HasName, HasDescription, HasLastUpdate, HasValidity, Serialize)]
pub struct ServiceCatalog {
    description: Option<String>,
    href: Option<Uri>,
    id: Option<String>,
    last_update: Option<TimeStamp>,
    lifecycle_status: Option<String>,
    name: Option<String>,
    valid_for: Option<TimePeriod>,
    version: Option<String>,
    // References
    related_party: Option<Vec<RelatedParty>>,
    category: Option<Vec<ServiceCategoryRef>>,
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
}

impl ServiceCatalog {
    /// Create a new Service Catalog instance
    pub fn new(name : impl Into<String>) -> ServiceCatalog {
        ServiceCatalog {
            name : Some(name.into()),
            lifecycle_status: Some(CAT_STATUS_NEW.into()),
            version: Some(CAT_VERS_NEW.into()),
            ..ServiceCatalog::create_with_time()
        }
    }

    /// Add a category to this Service Candidate by passing in a Category reference
    pub fn category(mut self, category : ServiceCategoryRef) -> ServiceCatalog {
        vec_insert(&mut self.category, category);   
        self 
    }
}

#[cfg(test)]
mod test {
    use crate::tmf633::service_category::ServiceCategory;

    use super::*;

    const CAT_NAME : &str = "CAT_NAME";
    const CAT_DESC : &str = "CAT_DESC";
    const CATEGORY_NAME : &str = "A_CATEGORY";

    #[test]
    fn test_servicecatalog_create() {
        let catalog = ServiceCatalog::new(CAT_NAME);

        assert_eq!(catalog.get_name(),CAT_NAME.to_string());
        assert_eq!(catalog.version.is_some(),true);
        assert_eq!(catalog.version.unwrap(),CAT_VERS_NEW.to_string());
        assert_eq!(catalog.lifecycle_status.is_some(),true);
        assert_eq!(catalog.lifecycle_status.unwrap(),CAT_STATUS_NEW.to_string());
    }

    #[test]
    fn test_servicecatalog_create_with_category() {
        let category = ServiceCategory::new(CATEGORY_NAME);

        let catalog = ServiceCatalog::new(CAT_NAME)
            .category(ServiceCategoryRef::from(category));

        assert_eq!(catalog.category.is_some(),true);
        assert_eq!(catalog.category.unwrap().len(),1);
    }

    #[test]
    fn test_servicecatalog_create_with_desc() {
        let catalog = ServiceCatalog::new(CAT_NAME)
            .description(CAT_DESC);

        assert_eq!(catalog.description.is_some(),true);
        assert_eq!(catalog.description.unwrap(),CAT_DESC.to_string());
    }

    #[test]
    fn test_servicecatalog_classpath() {

        let path = ServiceCatalog::get_class(); 

        assert_eq!(path,CLASS_PATH.to_string());
        // This feels silly but we should test since it was a bug
        assert_eq!(path.contains("Catalog"),true);
    }
}