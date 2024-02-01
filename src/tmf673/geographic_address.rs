//! Geographic Address Module

use serde::{Deserialize,Serialize};

use crate::{CreateTMF, HasId, HasName};
use tmflib_derive::{HasId,HasName};
use crate::LIB_PATH;
use super::MOD_PATH;

const CLASS_PATH : &str = "geographicAddress";


/// Geographic Sub Address
#[derive(Clone, Debug, Default, Deserialize, HasId, HasName, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeographicSubAddress {
    /// Building Name
    pub building_name: Option<String>,
    /// URI for SubAddress
    pub href: Option<String>,
    /// ID for Sub Address
    pub id: Option<String>,
    /// Level within building
    pub level_number: Option<String>,
    /// Level Type
    pub level_type: Option<String>,
    /// Name of Sub-Address
    pub name: Option<String>,
    /// Private Address Name
    pub private_street_name: Option<String>,
    /// Private Address Number
    pub private_street_number: Option<String>,
    /// Sub Address Type
    pub sub_address_type: Option<String>,
    /// Sub Unit Number
    pub sub_unit_number: Option<String>,
    /// Sub Unit
    pub sub_unit: Option<String>,
}
/// Geographic Address 
#[derive(Clone, Debug, Default, Deserialize, HasId,HasName, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeographicAddress {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTML Reference to this object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Name of this Address 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    street_name : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    street_nr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_or_province: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    street_type: Option<String>,
    // Reference Types
    #[serde(skip_serializing_if = "Option::is_none")]
    geographic_sub_address: Option<Vec<GeographicSubAddress>>,
}

impl GeographicAddress {
    /// Create a new Geographic Address
    pub fn new(name : impl Into<String>) -> GeographicAddress {
        let mut address = GeographicAddress::create();
        address.name = Some(name.into());
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