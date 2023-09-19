//! TMF620 Product Catalogue Management
//! 

use super::catalog::Catalog;
use super::category::Category;

/// Agreement Reference
pub struct AgreementRef {}

/// Bundled Product Offering
pub struct BundledProductOffering {}

/// Channel Reference
pub struct ChannelRef {}

/// Market Segment Refefence
pub struct MarketSegmentRef {}

/// Place Reference
pub struct PlaceRef {}

/// Resource Candidate Reference 
pub struct ResourceCandidateRef {}

/// Service Candidate Reference
pub struct ServiceCandidateRef {}

/// Service Level Agreement Reference 
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