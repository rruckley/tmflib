//! Geographic Site Module

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::convert::From;
use uuid::Uuid;

use super::MOD_PATH;
use crate::common::event::{Event, EventPayload};
use crate::common::related_party::RelatedParty;
use crate::tmf673::geographic_address::GeographicAddress;
use crate::{
    gen_code, HasDescription, HasId, HasName, HasValidity, TMFEvent, TimePeriod, LIB_PATH,
};
use tmflib_derive::{HasDescription, HasId, HasName, HasValidity};
const CLASS_PATH: &str = "geographicSite";
const DEFAULT_TZ: &str = "AEST";
const CODE_PREFIX: &str = "S-";
const CALENDAR_WEEKDAYS: &str = "weekdays";

/// Reference to a place
/// # Uses
/// Link to a place
/// Provide a place locally within the payload
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
            name: value.get_name(),
        }
    }
}

/// Relationship to other sites
#[derive(Clone, Debug, Default, Deserialize, Serialize, HasValidity)]
#[serde(rename_all = "camelCase")]
pub struct GeographicSiteRelationship {
    id: String,
    href: String,
    relationship_type: String,
    role: String,
    valid_for: Option<TimePeriod>,
}

/// Definition of start and finish hours
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HourPeriod {
    start_hour: String,
    end_hour: String,
}

/// Calendar entry defining periodic status for site, e.g. opening hours
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarPeriod {
    day: Option<String>,
    status: Option<String>,
    time_zone: Option<String>,
    hour_period: Option<Vec<HourPeriod>>,
}

impl CalendarPeriod {
    /// Generate standard business hours calendar
    pub fn business_hours() -> CalendarPeriod {
        CalendarPeriod {
            day: Some(CALENDAR_WEEKDAYS.to_string()),
            status: Some("open".to_string()),
            time_zone: Some(DEFAULT_TZ.to_string()),
            hour_period: Some(vec![HourPeriod {
                start_hour: "09:00 am".to_string(),
                end_hour: "05:00 pm".to_string(),
            }]),
        }
    }
}

/// Geographic Site
#[derive(Clone, Debug, Default, Deserialize, HasId, HasName, HasDescription, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeographicSite {
    /// Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HREF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Site Code
    pub code: Option<String>,
    /// Site Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Site Name
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    place: Option<Vec<PlaceRefOrValue>>,
    /// Site Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Relationships to other sides, e.g. floor , building,tenant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_relationship: Option<Vec<GeographicSiteRelationship>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    calendar: Option<Vec<CalendarPeriod>>,
    /// Customer / other parties related to this site
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

impl GeographicSite {
    /// Create a new Geographic Site with a name
    pub fn new(name: impl Into<String>) -> GeographicSite {
        let mut site = GeographicSite::create();
        site.name = Some(name.into());
        site.generate_code(None);
        site
    }
    /// Set the place on this Site
    pub fn place(mut self, place: PlaceRefOrValue) -> GeographicSite {
        match self.place.as_mut() {
            Some(v) => v.push(place),
            None => self.place = Some(vec![place]),
        }
        self
    }

    /// Set the code for this site
    pub fn code(mut self, code: String) -> GeographicSite {
        self.code = Some(code);
        self
    }

    /// Set the calendar for this site
    pub fn calendar(mut self, calendar: CalendarPeriod) -> GeographicSite {
        match self.calendar.as_mut() {
            Some(v) => v.push(calendar),
            None => self.calendar = Some(vec![calendar]),
        }
        self
    }

    /// Generate a new site code based on available fields
    pub fn generate_code(&mut self, offset: Option<u32>) {
        let (code, _hash) = gen_code(
            self.get_name(),
            self.get_id(),
            offset,
            Some(CODE_PREFIX.to_string()),
            None,
        );
        self.code = Some(code);
    }
}

/// Events for Geographic Site
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum GeographicSiteEventType {
    /// New Site Created
    GeographicSiteCreateEvent,
    /// Attribute changed on existing site
    GeographicSiteAttributeValueChangeEvent,
    /// Existing site changed state
    GeographicSiteStatusChangeEvent,
    /// Existing site deleted
    GeographicSiteDeleteEvent,
}

/// Container for the payload that generated the event
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeographicSiteEvent {
    /// Struct that this event relates to
    pub geographic_site: GeographicSite,
}

impl TMFEvent<GeographicSiteEvent> for GeographicSite {
    fn event(&self) -> GeographicSiteEvent {
        GeographicSiteEvent {
            geographic_site: self.clone(),
        }
    }
}

impl EventPayload<GeographicSiteEvent> for GeographicSite {
    type Subject = GeographicSite;
    type EventType = GeographicSiteEventType;

    fn to_event(&self, event_type: Self::EventType) -> Event<GeographicSiteEvent, Self::EventType> {
        let now = Utc::now();
        //let event_time = NaiveDateTime::from_timestamp_opt(now.timestamp(), 0).unwrap();
        let event_time = chrono::DateTime::from_timestamp(now.timestamp(), 0).unwrap();
        let desc = format!(
            "{:?} for {} [{}]",
            event_type,
            self.get_name(),
            self.get_id()
        );
        Event {
            correlation_id: self.code.clone(),
            description: Some(desc),
            domain: Some(GeographicSite::get_class()),
            event_id: Uuid::new_v4().to_string(),
            field_path: None,
            href: Some(self.get_href()),
            id: Some(self.get_id()),
            title: Some(self.get_name()),
            event_time: event_time.to_string(),
            priority: None,
            time_occurred: Some(event_time.to_string()),
            event_type,
            event: self.event(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tmf673::geographic_address::GeographicAddress;

    const SITE: &str = "ASites";
    const ADDRESS: &str = "AnAddress";
    const SITE_CODE: &str = "SiteCode";

    const PLACEREF_JSON: &str = "{
        \"id\" : \"P123\",
        \"href\" : \"http://example.com/tmf674/place/P123\",
        \"name\" : \"PlaceName\"
    }";

    const GEOSITEREL_JSON: &str = "{
        \"id\" : \"G123\",
        \"href\" : \"http://example.com/tmf674/site/G123\",
        \"relationshipType\" : \"RelationshipType\",
        \"role\" : \"Role\"
    }";

    const HOURPERIOD_JSON: &str = "{
        \"startHour\" : \"09:00\",
        \"endHour\" : \"17:00\"
    }";

    const CALENDAR_JSON: &str = "{
        \"day\" : \"Monday\",
        \"status\" :\"Status\"
    }";

    #[test]
    fn test_site_new_name() {
        let site = GeographicSite::new(SITE);

        assert_eq!(site.name, Some(SITE.into()));
    }

    #[test]
    fn test_site_new_place() {
        let place = GeographicAddress::new(ADDRESS);
        let place2 = place.clone();
        let site = GeographicSite::new(SITE).place(place.into());

        assert_eq!(site.place.unwrap()[0], PlaceRefOrValue::from(place2));
    }

    #[test]
    fn test_path() {
        let path = GeographicSite::get_class_href();

        assert_eq!(path.contains("geographicSiteManagement"), true);
    }

    #[test]
    fn test_site_business_hours() {
        let bus_hours = CalendarPeriod::business_hours();

        assert_eq!(bus_hours.day, Some(CALENDAR_WEEKDAYS.to_string()));
        assert_eq!(bus_hours.hour_period.is_some(), true);
    }

    #[test]
    fn test_site_code() {
        let site = GeographicSite::new(SITE).code(SITE_CODE.to_string());

        assert_eq!(site.code.is_some(), true);
        assert_eq!(site.code.unwrap().as_str(), SITE_CODE);
    }

    #[test]
    fn test_site_calendar() {
        let site = GeographicSite::new(SITE)
            .calendar(CalendarPeriod::business_hours())
            .calendar(CalendarPeriod::business_hours());

        assert_eq!(site.calendar.is_some(), true);
        assert_eq!(site.calendar.unwrap().len(), 2);
    }

    #[test]
    fn test_placeref_deserialize() {
        let placeref: PlaceRefOrValue =
            serde_json::from_str(PLACEREF_JSON).expect("Cannot parse PLACEREF_JSON");

        assert_eq!(placeref.id.as_str(), "P123");
        assert_eq!(placeref.name.as_str(), "PlaceName");
    }

    #[test]
    fn test_geositerel_deserialize() {
        let geositerel: GeographicSiteRelationship = serde_json::from_str(GEOSITEREL_JSON).unwrap();

        assert_eq!(geositerel.id.as_str(), "G123");
        assert_eq!(geositerel.relationship_type.as_str(), "RelationshipType");
        assert_eq!(geositerel.role.as_str(), "Role");
    }

    #[test]
    fn test_geosite_hasvalidity() {
        let mut geositerel = GeographicSiteRelationship::default();

        geositerel.set_validity(TimePeriod::period_30days());

        assert_eq!(geositerel.valid_for.is_some(), true);
    }

    #[test]
    fn test_hourperiod_deserialize() {
        let hourperiod: HourPeriod = serde_json::from_str(HOURPERIOD_JSON).unwrap();

        assert_eq!(hourperiod.start_hour.as_str(), "09:00");
        assert_eq!(hourperiod.end_hour.as_str(), "17:00");
    }

    #[test]
    fn test_calendar_deseralize() {
        let calendar: CalendarPeriod = serde_json::from_str(CALENDAR_JSON).unwrap();

        assert_eq!(calendar.day.is_some(), true);
        assert_eq!(calendar.day.unwrap().as_str(), "Monday");
        assert_eq!(calendar.status.is_some(), true);
        assert_eq!(calendar.status.unwrap().as_str(), "Status");
    }
}
