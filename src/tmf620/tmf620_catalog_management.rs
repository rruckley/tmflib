//! TMF620 Product Catalogue Management
//!
//! This module provides the management layer for objets defined within TMF620.

use crate::common::event::{Event,EventPayload};

use super::category::Category;
use super::product_specification::ProductSpecification;
use super::{catalog::{Catalog,CatalogEventType}, product_offering::ProductOffering};

use serde::{Deserialize, Serialize};


/// Root Struct to interact with catalogue
#[derive(Default)]
pub struct TMF620CatalogueManagement {
    catalogue: Option<Catalog>,
    category: Option<Category>,
    offers: Vec<ProductOffering>,
    catalogs: Vec<Catalog>,
    categories: Vec<Category>,
    specifications: Vec<ProductSpecification>,
    catalog_events: Vec<Event<Catalog,CatalogEventType>>,
}

impl TMF620CatalogueManagement {
    /// Create new instance for catalogue managment
    /// # Examples
    ///
    pub fn new() -> TMF620CatalogueManagement {
        TMF620CatalogueManagement {
            catalogue: None,
            category: None,
            offers: vec![],
            catalogs: vec![],
            categories: vec![],
            specifications: vec![],
            catalog_events: vec![],    
        }
    }

    /// Add a product offer into the catalog
    pub fn add_offer(&mut self, offer: ProductOffering) -> Result<String, String> {
        // Ideally we should validate that the category referenced in this PO is valid.
        self.offers.push(offer);
        Ok(String::from("Offer added"))
    }

    /// Add a catalog
    pub fn add_catalog(&mut self, catalog : Catalog) -> Result<String,String> {
        self.catalogs.push(catalog.clone());

        // When we add a new catalog, generate an event.
        let event = catalog.generate_event(CatalogEventType::CatalogCreateEvent);
        self.catalog_events.push(event);
        Ok(String::from("Catalog added"))
    }

    /// Added a category into the catalog
    pub fn add_category(&mut self, category: Category) -> Result<String, String> {
        self.categories.push(category);

        Ok(String::from("Category added"))
    }

    /// Add a specification
    pub fn add_specification(&mut self, specification: ProductSpecification) -> Result<String,String> {
        self.specifications.push(specification);
        Ok(String::from("Specification added"))
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
