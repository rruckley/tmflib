//! Catalogue Module
//!
//!
use crate::{
    HasId, 
    HasName,
    HasLastUpdate, 
    HasValidity,
    HasRelatedParty, 
    TimePeriod, 
    DateTime,
    TMFEvent,
    LIB_PATH,
    Uri,
};
use crate::tmf620::category::CategoryRef;
use crate::common::related_party::RelatedParty;
use crate::common::event::{Event,EventPayload};
use tmflib_derive::{HasLastUpdate,HasId,HasName,HasValidity,HasRelatedParty};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// URL Path components
use super::MOD_PATH;

const CLASS_PATH: &str = "catalog";
const CAT_VERS: &str = "1.0";

/// Catalogue
#[derive(Clone, Default, Debug, Deserialize,HasLastUpdate, HasId, HasName, HasValidity, HasRelatedParty, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    /// Non-optional fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTML reference to this object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Optional fields
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_update: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    /// Categories
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<Vec<CategoryRef>>,
    /// Related parties for party specific catalogs
    #[serde(skip_serializing_if = "Option::is_none")]
    related_party: Option<Vec<RelatedParty>>,
    // META
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@baseType")]
    base_type : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@schemaLocation")]
    schema_location: Option<Uri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    r#type : Option<String>,
}

impl Catalog {
    /// Create a new instance of catalog struct
    pub fn new(name : impl Into<String>) -> Catalog {
        let mut cat = Catalog::create_with_time();
        cat.name = Some(name.into());
        cat.version = Some(CAT_VERS.to_string());
        cat.category = Some(vec![]);
        cat.related_party = Some(vec![]);
        cat.valid_for = Some(TimePeriod::default());
        cat
    }

    /// Set the name for this Catalog
    pub fn name(mut self, name: String) -> Catalog {
        self.name = Some(name.clone());
        self
    }

    /// Add a category to a catalog
    pub fn add_category(&mut self, category: CategoryRef) {
        match self.category.as_mut() {
            Some(v) => v.push(category),
            None => self.category = Some(vec![category]),
        }
    }
}

/// Container for the payload that generated the event
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct CatalogEvent {
    /// Struct that this event relates to
    pub catalog: Catalog,
}

impl TMFEvent<CatalogEvent> for Catalog {
    fn event(&self) -> CatalogEvent {
        CatalogEvent {
            catalog : self.clone(),
        }
    }
}

impl EventPayload<CatalogEvent> for Catalog {
    type Subject = Catalog;
    type EventType = CatalogEventType;
    fn to_event(&self,event_type : CatalogEventType) -> Event<CatalogEvent,CatalogEventType> {       
        let now = Utc::now();
        let event_time = chrono::DateTime::from_timestamp(now.timestamp(),0).unwrap();
        Event {
            correlation_id: None,
            description: None,
            domain: Some(Catalog::get_class()),
            event_id: Uuid::new_v4().to_string(),
            field_path: None,
            href: self.href.clone(),
            id: self.id.clone(),
            title: self.name.clone(),
            event_time: event_time.to_string(),
            priority: None,
            time_occurred: None,
            event_type,
            event: self.event(),
        }
    }
}


/// Type of event fot he catalog events
#[derive(Debug,Deserialize,PartialEq, Serialize)]
pub enum CatalogEventType {
    /// Catalog has been created
    CatalogCreateEvent,
    /// Catalog has been deleted
    CatalogDeleteEvent,
    /// A Batch event has been triggered for a catalog
    CatalogBatchEvent,
}

// Notifications
/// Catalog created Event
pub struct CatalogCreateEvent {}
/// Catalog Deteted Event
pub struct CatalogDeleteEvent {}
/// Catalog Batch Event
pub struct CatalogBatchEvent {}

#[cfg(test)]
mod tests {

    const CAT_NAME : &str = "A Catalog";

    use crate::common::event::EventPayload;
    use crate::common::related_party::RelatedParty;
    use crate::tmf620::catalog::{CAT_VERS,CLASS_PATH};
    use crate::tmf632::organization_v4::Organization;

    use super::{Catalog, CatalogEvent, CatalogEventType};
    use crate::tmf620::category::{Category, CategoryRef};
    use crate::{HasId,HasName, HasValidity,HasRelatedParty,TimePeriod};

    const CAT_JSON : &str = "{
        \"name\" : \"CatalogueName\",
        \"@baseType\" : \"catalog\"
    }";

    const CAT_EVENT_TYPE_JSON : &str = "\"CatalogCreateEvent\"";

    const CATALOGEVENT_JSON : &str = "{
        \"catalog\" : {}
    }";

    #[test]
    fn test_cat_name() {
        let cat = Catalog::new(CAT_NAME);

        assert_eq!(cat.name, Some(String::from(CAT_NAME)));
    }

    #[test]
    fn test_cat_vers() {
        let cat = Catalog::new(CAT_NAME);

        assert_eq!(cat.version, Some(CAT_VERS.to_string()));
    }

    #[test]
    fn test_cat_class() {

        assert_eq!(Catalog::get_class(),CLASS_PATH.to_owned());
    }

    #[test]
    fn test_cat_rename() {
        let cat = Catalog::new(CAT_NAME)
            .name("NewName".to_string());

        assert_eq!(cat.get_name(),"NewName".to_string());
    }

    #[test]
    fn test_cat_add_cat() {
        let mut cat = Catalog::new(CAT_NAME);
        let category = Category::new("A Category");
        cat.add_category(CategoryRef::from(&category));

        assert_eq!(cat.category.is_some(),true);
    }

    #[test]
    fn test_cat_add_party() {
        let mut cat = Catalog::new(CAT_NAME);

        let org = Organization::new("An Organisation");

        cat.add_party(RelatedParty::from(&org));

        assert_eq!(cat.related_party.is_some(),true);
    }

    #[test]
    fn test_catalog_event() {
        let cat = Catalog::new(CAT_NAME);
        let event = cat.to_event(CatalogEventType::CatalogCreateEvent);

        assert_eq!(event.domain.unwrap(),Catalog::get_class());
        assert_eq!(event.href,cat.href);
        assert_eq!(event.id,cat.id);
        assert_eq!(event.title,cat.name);
    }

    #[test]
    fn test_catalog_deserialize() {
        let cat : Catalog = serde_json::from_str(CAT_JSON).unwrap();

        assert_eq!(cat.name.is_some(),true);
        assert_eq!(cat.get_name().as_str(),"CatalogueName");
    }

    #[test]
    fn test_catalog_hasvalidity() {
        let mut cat = Catalog::new(CAT_NAME);
        cat.set_validity(TimePeriod::period_30days());

        assert_eq!(cat.valid_for.is_some(),true);
        assert_eq!(cat.valid_for.unwrap().started(),true);
    }

    #[test]
    fn test_catalogeventtype_deserialize() {
        let eventtype : CatalogEventType = serde_json::from_str(CAT_EVENT_TYPE_JSON).unwrap();

        assert_eq!(eventtype,CatalogEventType::CatalogCreateEvent);
    }

    #[test]
    fn test_catalogevent_deserialize() {
        let _catalogevent : CatalogEvent = serde_json::from_str(CATALOGEVENT_JSON).unwrap();
    }

    #[test]
    fn test_catalogeventtype_display() {
        let catalogeventtype = CatalogEventType::CatalogCreateEvent;

        println!("{:?}",catalogeventtype);

        let catalogeventtype = CatalogEventType::CatalogBatchEvent;

        println!("{:?}",catalogeventtype);

        let catalogeventtype = CatalogEventType::CatalogDeleteEvent;

        println!("{:?}",catalogeventtype);
    }


}
