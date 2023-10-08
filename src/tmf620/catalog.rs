//! Catalogue Module
//!
//!
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
#[derive(Clone,Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    /// Non-optional fields
    pub id: Option<String>,
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

impl Catalog {
    /// Create a new instance of catalog struct
    pub fn new() -> Catalog {
        Catalog::default()
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

impl std::default::Default for Catalog {
    fn default() -> Catalog {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}", LIB_PATH, MOD_PATH, CAT_PATH, id);
        let now = Utc::now();
        let time = NaiveDateTime::from_timestamp_opt(now.timestamp(), 0).unwrap();
        Catalog {
            id : Some(id),
            href : Some(href),
            catalog_type: None,
            description: None,
            last_update: Some(time.to_string()),
            lifecycle_status: None,
            name: None,
            version: Some(CAT_VERS.to_string()),
            valid_for: None,
            category: Some(vec![]),
            related_party: None,
        }
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

#[derive(Debug,Deserialize,Serialize)]
pub enum CatalogEventType {
    CatalogCreateEvent,
    CatalogDeleteEvent,
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
pub struct CatalogCreateEvent {}
pub struct CatalogDeleteEvent {}
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
