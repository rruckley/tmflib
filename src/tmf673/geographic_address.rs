//! Geographic Address Module

use serde::{Deserialize,Serialize};
use uuid::Uuid;

use crate::CreateTMF;
use crate::HasId;
use crate::LIB_PATH;

use super::MOD_PATH;

const GEO_PATH : &str = "address";


/// Geographic Address 
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeographicAddress {
    /// Unique Id
    pub id: Option<String>,
    /// HTML Reference to this object
    pub href: Option<String>,
    /// Name of this Address 
    pub name : String,
    locality: Option<String>,
    street_name : Option<String>,
    street_nr: Option<String>,
    state_or_province: Option<String>,
    street_type: Option<String>,
}

impl GeographicAddress {
    /// Create a new Geographic Address
    pub fn new(name : String) -> GeographicAddress {
        let mut address = GeographicAddress::create();
        address.name = name;
        address
    }

    /// Set the street for this Address
    pub fn street(mut self, street: &str) -> GeographicAddress {
        if street.split(' ').count() > 1 {
            // Attempt to split string like "Lumeah Ave" into two parts
        }
        self.street_name = Some(street.to_string());
        self
    }
    /// Set the street type for this address
    pub fn street_type(mut self, street_type: &str) -> GeographicAddress {
        self.street_type = Some(street_type.to_string());
        self
    }
    /// Set the street number of this address
    pub fn number(mut self, number : &str) -> GeographicAddress {
        self.street_nr = Some(number.to_string());
        self
    }
    /// Set the suburb (locality) for this address
    pub fn suburb(mut self, suburb: &str) -> GeographicAddress {
        self.locality = Some(suburb.to_string());
        self
    }
    /// Set the state (or province) for this address
    pub fn state(mut self, state : &str) -> GeographicAddress {
        self.state_or_province = Some(state.to_string());
        self
    }
}

impl HasId for GeographicAddress {
    fn generate_href(&mut self) {
        let href = format!("{}/{}",GeographicAddress::get_class_href(),self.get_id());
        self.href = Some(href);
    }
    fn generate_id(&mut self) {
        let id = Uuid::new_v4().as_simple().to_string();
        self.id = Some(id);
        self.generate_href();    
    }
    fn get_href(&self) -> String {
        self.href.as_ref().unwrap().clone()    
    }
    fn get_class_href() -> String {
        format!("/{}/{}/{}",LIB_PATH,MOD_PATH,GeographicAddress::get_class())    
    }
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone()    
    }
    fn get_class() -> String {
        GEO_PATH.to_owned()
    }
}

impl CreateTMF<GeographicAddress> for GeographicAddress {}