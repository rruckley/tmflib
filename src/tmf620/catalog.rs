//! Catalogue Module
//!
//!
use crate::{HasId, CreateTMF, HasName, CreateTMFWithTime,HasLastUpdate, TimePeriod};
use crate::tmf620::category::CategoryRef;
use crate::common::related_party::RelatedParty;
use crate::common::event::{Event,EventPayload};
use tmflib_derive::HasId;

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
#[derive(Clone, Default, Debug, Deserialize, HasId, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    /// Non-optional fields
    pub id: Option<String>,
    /// HTML reference to this object
    pub href: Option<String>,
    /// Optional fields
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_update: Option<String>,
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

impl HasLastUpdate for Catalog {
    fn set_last_update(&mut self, time : impl Into<String>) {
        self.last_update = Some(time.into());
    }
}

impl HasName for Catalog {
    fn get_name(&self) -> String {
        self.name.clone().unwrap_or("NoName".to_string())
    }
}

impl CreateTMFWithTime<Catalog> for Catalog {

}

impl Catalog {
    /// Create a new instance of catalog struct
    pub fn new(name : impl Into<String>) -> Catalog {
        let mut cat = Catalog::create_with_time();
        cat.name = Some(name.into());
        cat.version = Some(CAT_VERS.to_string());
        cat.category = Some(vec![]);
        cat.related_party = Some(vec![]);
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

impl EventPayload<Catalog,CatalogEventType> for Catalog {
    fn generate_event(&self,event_type : CatalogEventType) -> crate::common::event::Event<Catalog,CatalogEventType> {       
        let now = Utc::now();
        let event_time = NaiveDateTime::from_timestamp_opt(now.timestamp(), 0).unwrap();
        Event {
            correlation_id: None,
            description: None,
            domain: None,
            event_id: Uuid::new_v4().to_string(),
            field_path: None,
            href: self.href.clone(),
            id: self.id.clone(),
            title: self.name.clone(),
            event_time: event_time.to_string(),
            priority: None,
            time_occurred: None,
            event_type,
            event: self.clone(),
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

    use crate::tmf620::catalog::{CAT_VERS,CLASS_PATH};

    use super::Catalog;
    use crate::HasId;

    #[test]
    fn test_cat_name() {
        let cat = Catalog::new("MyCatalog");

        assert_eq!(cat.name, Some(String::from("MyCatalog")));
    }

    #[test]
    fn test_cat_vers() {
        let cat = Catalog::new("MyCatalog");

        assert_eq!(cat.version, Some(CAT_VERS.to_string()));
    }

    #[test]
    fn test_cat_class() {

        assert_eq!(Catalog::get_class(),CLASS_PATH.to_owned());
    }
}
