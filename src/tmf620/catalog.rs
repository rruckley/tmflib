//! Catalogue Module
//! 
//! 
use crate::tmf620::party::RelatedParty;
use crate::tmf620::category::CategoryRef;

use serde::{Deserialize,Serialize};
use uuid::Uuid;
use chrono::naive::NaiveDateTime;
use chrono::Utc;

// URL Path components
use super::LIB_PATH;
use super::MOD_PATH;


const CAT_PATH : &str = "catalog";
const CAT_VERS : &str = "1.0";

/// Catalogue
#[derive(Debug,Deserialize, Serialize)]
pub struct Catalog {
    /// Non-optional fields
    id              : String,
    href            : String,
    /// Optional fields
    catalog_type    : Option<String>,
    description     : Option<String>,
    last_update     : Option<String>,
    lifecycle_status : Option<String>,
    name            : Option<String>,
    version         : Option<String>,
    valid_for       : Option<String>,
    /// Categories
    categories       : Option<Vec<CategoryRef>>,
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

    /// Add a category to a catalog
    pub fn add_category(self, category : CategoryRef) -> Result<String,String> {
        match self.categories {
            None => {
                Err(String::from("Missing category Vec"))
            },
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
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,CAT_PATH,id);
        let now = Utc::now();
        let time = NaiveDateTime::from_timestamp_opt(now.timestamp(), 0).unwrap();
        Catalog {
            id          : id,
            href        : href,
            catalog_type : None,
            description : None,
            last_update : Some(time.to_string()),
            lifecycle_status : None,
            name        : None,
            version     : Some(CAT_VERS.to_string()),
            valid_for   : None,
            categories    : Some(vec![]),
            related_party : None,
        }
    }
}

#[cfg(test)]
mod tests {
    
    use crate::tmf620::catalog::CAT_VERS;

    use super::Catalog;
    #[test]
    fn test_cat_default() {
        let cat = Catalog::new();

        assert_eq!(cat.name , None);
    }

    #[test]
    fn test_cat_name() {
        let cat = Catalog::new().name(String::from("MyCatalog"));

        assert_eq!(cat.name , Some(String::from("MyCatalog")));
    }

    #[test]
    fn test_cat_vers() {
        let cat = Catalog::new().name(String::from("MyCatalog"));

        assert_eq!(cat.version , Some(CAT_VERS.to_string()));
    }


}