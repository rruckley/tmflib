//! Category Module

use crate::tmf620::product_offering::ProductOfferingRef;

use serde::{Deserialize,Serialize};
use uuid::Uuid;

use super::MOD_PATH;

const CAT_PATH : &str = "catalog";

/// Category Resource
#[derive(Debug,Deserialize, Serialize)]
pub struct Category {
    // Scalar fields
    /// Id
    pub id : Option<String>,
    /// HREF where object is located
    pub href : Option<String>,
    /// Description
    pub description : Option<String>,
    /// Is this the root of a heirarchy of categories?
    pub is_root : bool,
    /// When was this object last updated?
    pub last_update : Option<String>,
    /// What is the status of this object?
    pub lifecycle_status : Option<String>,
    /// Name
    pub name : Option<String>,
    /// Id of parent in the heirarchy
    pub parent_id : Option<String>,
    /// Version of this record
    pub version : Option<String>,
    /// How long his this object valid for?
    pub valid_for : Option<String>,
    /// Subcategory
    pub sub_category : Option<Vec<CategoryRef>>,
    /// Product Offering
    pub product_offering : Option<Vec<ProductOfferingRef>>,
}

impl Category {
    /// Create a new instance of the Category struct
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::category::Category;
    /// let cat = Category::new();
    /// ```
    pub fn new(name : String) -> Category {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}",MOD_PATH,CAT_PATH,id);
        Category { 
            id          : Some(id),
            href        : Some(href),
            description : None,
            is_root     : false,
            last_update : None,
            lifecycle_status : None,
            name        : Some(name.clone()),
            parent_id   : None,
            version     : None,
            valid_for   : None,
            sub_category: None, 
            product_offering: None
        }
    }

    /// Set the name
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::category::Category;
    /// let cat = Category::new()
    ///     .name(String::from("Components"));
    /// ```
    pub fn name(mut self, name : String) -> Category {
        self.name = Some(name);
        self
    }

    /// Set the description of this category
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::category::Category;
    /// let cat = Category::new()
    ///     .description(String::from("Library of product components"));
    /// ```
    pub fn description(mut self, description :  String) -> Category {
        self.description = Some(description);
        self
    }

}

/// Category Reference
#[derive(Debug,Deserialize, Serialize)]
pub struct CategoryRef {
    id      : String,
    href    : String,
    name    : String,
    version : String,   
}