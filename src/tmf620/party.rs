//! Party Module
//! 
//! 

use serde::{Deserialize,Serialize};
/// Related Party
#[derive(Debug,Deserialize,Serialize)]
pub struct RelatedParty {
    id      : String,
    href    : String,
    name    : String,
    role    : String,   
}