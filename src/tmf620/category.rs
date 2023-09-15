//! Category Module

use crate::tmf620::product_offering::ProductOfferingRef;

/// Category Resource
pub struct Category {
    /// Subcategory
    pub sub_category : Vec<CategoryRef>,
    /// Product Offering
    pub product_offering : Vec<ProductOfferingRef>,
}

/// Category Reference
pub struct CategoryRef {
    id      : String,
    href    : String,
    name    : String,
    version : String,   
}