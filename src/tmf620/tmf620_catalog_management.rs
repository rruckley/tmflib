//! TMF620 Product Catalogue Management
//! 

use super::{catalog::Catalog, product_offering::ProductOffering};
use super::category::Category;

use serde::{Deserialize,Serialize};

/// Agreement Reference
#[derive(Debug,Deserialize,Serialize)]
pub struct AgreementRef {}

/// Bundled Product Offering
#[derive(Debug,Deserialize,Serialize)]
pub struct BundledProductOffering {}

/// Channel Reference
#[derive(Debug,Deserialize,Serialize)]
pub struct ChannelRef {}

/// Market Segment Refefence
#[derive(Debug,Deserialize,Serialize)]
pub struct MarketSegmentRef {}

/// Place Reference
#[derive(Debug,Deserialize,Serialize)]
pub struct PlaceRef {}

/// Resource Candidate Reference 
#[derive(Debug,Deserialize,Serialize)]
pub struct ResourceCandidateRef {}


/// Service Candidate Reference
#[derive(Debug,Deserialize,Serialize)]
pub struct ServiceCandidateRef {}

/// Service Level Agreement Reference 
#[derive(Debug,Deserialize,Serialize)]
pub struct SLARef {}

/// Root Struct to interact with catalogue
pub struct CatalogueManagement {
    catalogue   : Option<Catalog>,
    category    : Option<Category>,
    offers      : Vec<ProductOffering>,
    categories  : Vec<Category>,
}

impl CatalogueManagement {
    /// Create new instance for catalogue managment
    /// # Examples
    /// 
    pub fn new() -> CatalogueManagement {
        CatalogueManagement { 
            catalogue : None,
            category  : None,
            offers    : vec![],
            categories : vec![],
        }
    }

    /// Add a product offer into the catalog
    pub fn add_offer(&mut self, offer : ProductOffering) -> Result<String,String> {
        // Ideally we should validate that the category referenced in this PO is valid.
        self.offers.push(offer);
        Ok(String::from("Offer added"))
    }

    /// Added a category into the catalog
    pub fn add_category(&mut self, category : Category) -> Result<String,String> {
        self.categories.push(category);
        Ok(String::from("Category added"))
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
    pub fn category(mut self) -> Category {
        match self.category {
            Some(c) => c,
            None => {
                self.category = Some(Category::new(String::from("root")));
                self.category.unwrap()
            }
        }
    }
}