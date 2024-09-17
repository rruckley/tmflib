//! Resource Catalog Module

use serde::{Deserialize,Serialize};

const CLASS_PATH : &str = "catalog";

use super::MOD_PATH;

use crate::{
    LIB_PATH,
    HasId,
    HasName,
    HasValidity,
    HasLastUpdate,
    HasDescription,
    Uri,
    DateTime,
    TimePeriod,
};

use tmflib_derive::{
    HasId,
    HasName,
    HasLastUpdate,
    HasDescription,
    HasValidity,
};

/// Resource Catalog structure
#[derive(Clone,Default,Debug,Deserialize,HasId,HasName,HasLastUpdate,HasDescription,HasValidity,Serialize)]
pub struct Catalog {
    /// Cataloge Type
    pub catalog_type : Option<String>,
    /// Cataloge Description
    pub description: Option<String>,
    /// HTTP Reference
    pub href: Option<Uri>,
    /// Unique Identier
    pub id: Option<String>,
    /// Last Update
    pub last_update: Option<DateTime>,
    /// Lifecycle status
    pub lifecycle_status: Option<String>,
    /// Name
    pub name: Option<String>,
    /// Validity
    pub valid_for: Option<TimePeriod>,
    /// Version
    pub version: Option<String>,
}

impl Catalog {
    /// Create a new catalog
    pub fn new(name : impl Into<String>) -> Catalog {
        Catalog {
            name : Some(name.into()),
            ..Catalog::create()
        }
    }
}

#[cfg(test)]
mod test {
    use super::Catalog;
    use crate::HasDescription;

    const CATALOG_NAME : &str = "CatalogueName";
    const CATALOG_DESC : &str = "CatalogueDescription";

    #[test]
    fn test_catalog_description() {
        let catalog = Catalog::new(CATALOG_NAME)
            .description(CATALOG_DESC);

        assert_eq!(catalog.description.is_some(),true);
        assert_eq!(catalog.get_description().as_str(),CATALOG_DESC);
    }
}