//! TMF620 Product Catalogue Management
//! 

use super::catalog::Catalog;
use super::category::Category;

use serde::{Deserialize,Serialize};

/// Agreement Reference
#[derive(Deserialize,Serialize)]
pub struct AgreementRef {}

/// Bundled Product Offering
#[derive(Deserialize,Serialize)]
pub struct BundledProductOffering {}

/// Channel Reference
#[derive(Deserialize,Serialize)]
pub struct ChannelRef {}

/// Market Segment Refefence
#[derive(Deserialize,Serialize)]
pub struct MarketSegmentRef {}

/// Place Reference
#[derive(Deserialize,Serialize)]
pub struct PlaceRef {}

/// Resource Candidate Reference 
#[derive(Deserialize,Serialize)]
pub struct ResourceCandidateRef {}


/// Service Candidate Reference
#[derive(Deserialize,Serialize)]
pub struct ServiceCandidateRef {}

/// Service Level Agreement Reference 
#[derive(Deserialize,Serialize)]
pub struct SLARef {}

/// Root Struct to interact with catalogue
pub struct CatalogueManagement {
    catalogue   : Option<Catalog>,
    category    : Option<Category>,
}

impl CatalogueManagement {
    /// Create new instance for catalogue managment
    /// # Examples
    /// 
    pub fn new() -> CatalogueManagement {
        CatalogueManagement { 
            catalogue : None,
            category  : None,
        }
    }

    /// Get an instance of Catalog struct for interacting with catalogues
    pub fn catalog(mut self) -> Catalog {
        match self.catalogue {
            Some(c) => c,
            None => {
                self.catalogue = Some(Catalog::new());
                self.catalogue.unwrap()
            }
        }
    }

    /// Get an instane of category struct for interacting with categories
    pub fn catalegory(mut self) -> Category {
        match self.category {
            Some(c) => c,
            None => {
                self.category = Some(Category::new());
                self.category.unwrap()
            }
        }
    }
}