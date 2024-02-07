//! Geographic Site Module

use serde::{Deserialize,Serialize};
use std::convert::From;

use crate::{HasName,HasId,CreateTMF,HasValidity, TimePeriod};
use tmflib_derive::{HasId, HasValidity};
use crate::tmf673::geographic_address::GeographicAddress;
use crate::common::related_party::RelatedParty;
use crate::LIB_PATH;
use super::MOD_PATH;
const CLASS_PATH: &str = "geographicSite";
const DEFAULT_TZ : &str = "AEST";

/// Refernce to a place
/// # Uses
/// Link to a place
/// Provide a place locally within the payload
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceRefOrValue {
    id: String,
    href: String,
    name: String,
}

impl From<GeographicAddress> for PlaceRefOrValue {
    fn from(value: GeographicAddress) -> Self {
        PlaceRefOrValue { 
            id: value.get_id(), 
            href: value.get_href(), 
            name: value.get_name() 
        }
    }
}


/// Relationship to other sites
#[derive(Clone, Debug, Default, Deserialize, Serialize, HasValidity)]
pub struct GeographicSiteRelationship {
    id : String,
    href : String,
    relationship_type : String,
    role : String,
    valid_for: Option<TimePeriod>,
}

/// Definition of start and finish hours
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HourPeriod {
    start_hour : String,
    end_hour : String,
}

/// Calendar entry defining periodic status for site, e.g. opening hours
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CalendarPeriod {
    day : Option<String>,
    status : Option<String>,
    time_zone : Option<String>,
    hour_period : Option<Vec<HourPeriod>>,
}

impl CalendarPeriod {
    /// Generate standard business hours calendar
    pub fn business_hours() -> CalendarPeriod {
    CalendarPeriod {
        day : Some("weekdays".to_string()),
        status : Some("open".to_string()),
        time_zone : Some(DEFAULT_TZ.to_string()),
        hour_period : Some(
            vec![HourPeriod{
                start_hour : "09:00 am".to_string(),
                end_hour : "05:00 pm".to_string(),
            }
            ]
        )
        }
    }
}

/// Geographic Site
#[derive(Clone, Debug, Default, Deserialize, HasId, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeographicSite {
    /// Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HREF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,#[serde(skip_serializing_if = "Option::is_none")]
    /// Site Code
    
    pub code : Option<String>,
    /// Site Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description : Option<String>,
    /// Site Name
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    place: Option<PlaceRefOrValue>,
    /// Site Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status : Option<String>,
    /// Relationships to other sides, e.g. floor , building,tenant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_relationship: Option<Vec<GeographicSiteRelationship>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar : Option<Vec<CalendarPeriod>>,
    /// Customer / other parties related to this site
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

impl GeographicSite {
    /// Create a new Geographic Site with a name
    pub fn new(name : impl Into<String>) -> GeographicSite {
        let mut site = GeographicSite::create();
        site.name = name.into();
        site.calendar = Some(vec![]);
        site
    }
    /// Set the place on this Site
    pub fn place(mut self, place : PlaceRefOrValue) -> GeographicSite {
        self.place = Some(place);
        self    
    }

    /// Set the code for this site
    pub fn code(mut self, code : String) -> GeographicSite {
        self.code = Some(code);
        self
    }

    /// Set the calendar for this site
    pub fn calendar(mut self, calendar : CalendarPeriod) -> GeographicSite {
        self.calendar.as_mut().unwrap().push(calendar);
        self
    }
}

impl HasName for GeographicSite {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

