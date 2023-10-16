//! Catalogue Module
//!
//!
use crate::{HasId, CreateTMFWithTime,HasLastUpdate};
use crate::tmf620::category::CategoryRef;
use crate::tmf620::party::RelatedParty;
use crate::common::event::{Event,EventPayload};

use chrono::naive::NaiveDateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// URL Path components
use super::LIB_PATH;
use super::MOD_PATH;

const CAT_PATH: &str = "catalog";
const CAT_VERS: &str = "1.0";

/// Catalogue
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
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
    valid_for: Option<String>,
    /// Categories
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<Vec<CategoryRef>>,
    /// Related parties
    #[serde(skip_serializing_if = "Option::is_none")]
    related_party: Option<Vec<RelatedParty>>,
}

impl HasLastUpdate for Catalog {
    fn set_last_update(&mut self, time : String) {
        self.last_update = Some(time);
    }
}

impl CreateTMFWithTime<Catalog> for Catalog {

}

impl Catalog {
    /// Create a new instance of catalog struct
    pub fn new() -> Catalog {
        let mut cat = Catalog::create_with_time();
        cat.version = Some(CAT_VERS.to_string());
        cat.category = Some(vec![]);
        cat
    }

    /// Set the name for this Catalog
    pub fn name(mut self, name: String) -> Catalog {
        self.name = Some(name.clone());
        self
    }

    /// Add a category to a catalog
    pub fn add_category(self, category: CategoryRef) -> Result<String, String> {
        match self.category {
            None => Err(String::from("Missing category Vec")),
            Some(mut c) => {
                c.push(category);
                Ok(String::from("Category added"))
            }
        }
    }
}

impl HasId for Catalog {
    fn get_id(&mut self) -> String {
        if self.id.is_none() {
            self.generate_id();
        }
        self.id.as_ref().unwrap().clone()
    }

    fn get_href(&mut self) -> String {
        if self.href.is_none() {
            self.generate_href();
        }
        self.href.as_ref().unwrap().clone()
    }

    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,CAT_PATH,self.get_id());
        self.href = Some(href);    
    }

    fn generate_id(&mut self) {
        // No return type for now

        let id = Catalog::get_uuid();
        self.id = Some(id);
        self.generate_href();
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

    use crate::tmf620::catalog::CAT_VERS;

    use super::Catalog;
    #[test]
    fn test_cat_default() {
        let cat = Catalog::new();

        assert_eq!(cat.name, None);
    }

    #[test]
    fn test_cat_name() {
        let cat = Catalog::new().name(String::from("MyCatalog"));

        assert_eq!(cat.name, Some(String::from("MyCatalog")));
    }

    #[test]
    fn test_cat_vers() {
        let cat = Catalog::new().name(String::from("MyCatalog"));

        assert_eq!(cat.version, Some(CAT_VERS.to_string()));
    }
}
