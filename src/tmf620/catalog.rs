//! Catalogue Module
//! 
//! 
use crate::tmf620::party::RelatedParty; 

/// Catalogue
pub struct Catalog {
    id              : String,
    href            : String,
    catalog_type    : String,
    description     : String,
    last_upodate    : String,
    lifecycle_status : String,
    name            : String,
    version         : String,
    valid_for       : String,
    /// Categories
    category        : Vec<CategoryRef>,
    /// Related parties
    related_party   : Vec<RelatedParty>,
}