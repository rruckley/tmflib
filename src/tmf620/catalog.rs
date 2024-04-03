//! Catalogue Module
//!
//!
use crate::{CreateTMF, CreateTMFWithTime, DateTime, HasId, HasLastUpdate, HasName, HasValidity, TMFEvent, TimePeriod};
use crate::tmf620::category::CategoryRef;
use crate::common::related_party::RelatedParty;
use crate::common::event::{Event,EventPayload};
use tmflib_derive::{HasLastUpdate,HasId,HasName,HasValidity};

use chrono::naive::NaiveDateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// URL Path components
use super::LIB_PATH;
use super::MOD_PATH;

const CLASS_PATH: &str = "catalog";
const CAT_VERS: &str = "1.0";

/// Catalogue
#[derive(Clone, Default, Debug, Deserialize,HasLastUpdate, HasId, HasName, HasValidity, Serialize)]
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
        self.category.as_mut().unwrap().push(category);
    }

    /// Add party to a catalog
    pub fn add_party(&mut self, party : RelatedParty) {
        self.related_party.as_mut().unwrap().push(party);
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
        let event_time = NaiveDateTime::from_timestamp_opt(now.timestamp(), 0).unwrap();
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
#[derive(Debug,Deserialize,Serialize)]
pub enum CatalogEventType {
    /// Catalog has been created
    CatalogCreateEvent,
    /// Catalog has been deleted
    CatalogDeleteEvent,
    /// A Batch event has been triggered for a catalog
    CatalogBatchEvent,
}

impl std::fmt::Display for CatalogEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CatalogEventType::CatalogCreateEvent => write!(f, "CatalogCreateEvent"),
            CatalogEventType::CatalogDeleteEvent => write!(f, "CatalogDeleteEvent"),
            CatalogEventType::CatalogBatchEvent => write!(f, "CatalogBatchEvent"),
        }
    }
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

    use crate::tmf620::catalog::{CAT_VERS,CLASS_PATH};

    use super::Catalog;
    use crate::HasId;

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
}
