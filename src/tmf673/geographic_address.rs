//! Geographic Address Module

use serde::{Deserialize,Serialize};
use uuid::Uuid;

use crate::CreateTMF;
use crate::HasId;
use crate::LIB_PATH;

use super::MOD_PATH;

const GEO_PATH : &str = "address";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GeographicAddress {
    pub id: Option<String>,
    pub href: Option<String>,
    pub name : String,
    locality: Option<String>,
    street_name : Option<String>,
    street_nr: Option<String>,
    state_or_province: Option<String>,
    street_type: Option<String>,
}

impl GeographicAddress {
    pub fn new(name : String) -> GeographicAddress {
        let mut address = GeographicAddress::create();
        address.name = name;
        address
    }

    pub fn street(mut self, street: &str) -> GeographicAddress {
        if street.split(' ').count() > 1 {
            // Attempt to split string like "Lumeah Ave" into two parts
        }
        self.street_name = Some(street.to_string());
        self
    }
    pub fn street_type(mut self, street_type: &str) -> GeographicAddress {
        self.street_type = Some(street_type.to_string());
        self
    }
    pub fn number(mut self, number : &str) -> GeographicAddress {
        self.street_nr = Some(number.to_string());
        self
    }
    pub fn suburb(mut self, suburb: &str) -> GeographicAddress {
        self.locality = Some(suburb.to_string());
        self
    }
    pub fn state(mut self, state : &str) -> GeographicAddress {
        self.state_or_province = Some(state.to_string());
        self
    }
}

impl HasId for GeographicAddress {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,GEO_PATH,self.get_id());
        self.href = Some(href);
    }
    fn generate_id(&mut self) {
        let id = Uuid::new_v4().as_simple().to_string();
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

impl CreateTMF<GeographicAddress> for GeographicAddress {}