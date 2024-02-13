//! Related Place Module


use serde::{Deserialize, Serialize};
use std::convert::From;
#[cfg(feature = "v4")]
use crate::tmf674::geographic_site_v4::GeographicSite;
#[cfg(feature = "v5")]
use crate::tmf674::geographic_site_v5::GeographicSite;
use crate::{HasId,HasName};

/// Reference to a place (TMF673, TMF674, TMF674)
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
            name: value.get_name(), 
            href: value.get_href(), 
            id: value.get_id(), 
            base_type: Some(String::from("GeographicSite")), 
            schema_location: None, 
            r#type: None 
        }
    }
}