//! Geographic Site Module

use serde::{Deserialize,Serialize};
use uuid::Uuid;
use std::convert::From;

use crate::CreateTMF;
use crate::HasId;
use crate::tmf673::geographic_address::GeographicAddress;
use crate::LIB_PATH;
use super::MOD_PATH;
const GEO_PATH: &str = "site";

/// Refernce to a place
/// # Uses
/// Link to a place
/// Provide a place locally within the payload
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlaceRefOrValue {
    id: String,
    href: String,
    name: String,
}

impl From<GeographicAddress> for PlaceRefOrValue {
    fn from(value: GeographicAddress) -> Self {
        PlaceRefOrValue { 
            id: value.id.as_ref().unwrap().clone(), 
            href: value.href.as_ref().unwrap().clone(), 
            name: value.name.clone() 
        }
    }
}

/// Geographic Site
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GeographicSite {
    id: Option<String>,
    href: Option<String>,
    name: String,
    place: Option<PlaceRefOrValue>,
}

impl GeographicSite {
    /// Create a new Geographic Site with a name
    pub fn new(name : String) -> GeographicSite {
        let mut site = GeographicSite::create();
        site.name = name;
        site
    }
    /// Set the place on this Site
    pub fn place(mut self, place : PlaceRefOrValue) -> GeographicSite {
        self.place = Some(place);
        self    
    }
}

impl CreateTMF<GeographicSite> for GeographicSite {}

impl HasId for GeographicSite {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,GEO_PATH,self.get_id());
        self.href = Some(href);    
    }
    fn generate_id(&mut self) {
        let id = Uuid::new_v4().simple().to_string();
        self.id = Some(id);
        self.generate_href();    
    }
    fn get_href(&mut self) -> String {
        self.href.as_ref().unwrap().clone()    
    }
    fn get_id(&mut self) -> String {
        self.id.as_ref().unwrap().clone()    
    }
}

