//! Related Place Module


use serde::{Deserialize, Serialize};
use std::convert::From;
use crate::tmf674::geographic_site::GeographicSite;
use crate::HasId;

/// Reference to a place (TMF673, TMF674, TMF674)
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RelatedPlaceRefOrValue {
    referred_type : String,
    name: String,
    href: String,
    id: String,
    base_type: Option<String>,
    schema_location: Option<String>,
    r#type: Option<String>,
}

impl From<GeographicSite> for RelatedPlaceRefOrValue {
    fn from(value: GeographicSite) -> Self {
        RelatedPlaceRefOrValue { 
            referred_type: GeographicSite::get_class(), 
            name: value.name.clone(), 
            href: value.href.as_ref().unwrap().clone(), 
            id: value.id.as_ref().unwrap().clone(), 
            base_type: Some(String::from("GeographicSite")), 
            schema_location: None, 
            r#type: None 
        }
    }
}