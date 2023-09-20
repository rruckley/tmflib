//! Catalogue Module
//! 
//! 
use crate::tmf620::party::RelatedParty;
use crate::tmf620::category::CategoryRef;

use serde::{Deserialize,Serialize};
use uuid::Uuid;

use super::MOD_PATH;

const CAT_PATH : &str = "catalog";

/// Catalogue
#[derive(Deserialize, Serialize)]
pub struct Catalog {
    id              : Option<String>,
    href            : Option<String>,
    catalog_type    : Option<String>,
    description     : Option<String>,
    last_update     : Option<String>,
    lifecycle_status : Option<String>,
    name            : Option<String>,
    version         : Option<String>,
    valid_for       : Option<String>,
    /// Categories
    category        : Option<Vec<CategoryRef>>,
    /// Related parties
    related_party   : Option<Vec<RelatedParty>>,
}

impl Catalog {
    /// Create a new instance of catalog struct
    pub fn new() -> Catalog {
        Catalog::default()
    }

    /// Set the name for this Catalog
    pub fn name(mut self, name : String) -> Catalog {
        self.name = Some(name.clone());
        self
    }
}

impl std::default::Default for Catalog {
    fn default() -> Catalog {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}",MOD_PATH,CAT_PATH,id);
        Catalog {
            id          : Some(id),
            href        : Some(href),
            catalog_type : None,
            description : None,
            last_update : None,
            lifecycle_status : None,
            name        : None,
            version     : None,
            valid_for   : None,
            category    : None,
            related_party : None,
        }
    }
}