//! Product Offering Price Module

use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::MOD_PATH;
use crate::{HasId,HasName, HasLastUpdate, LIB_PATH,TMFEvent,HasValidity, TimePeriod};
use crate::common::money::Money;
use crate::common::tax_item::TaxItem;
use crate::common::event::{Event,EventPayload};
use tmflib_derive::{HasId,HasLastUpdate,HasName, HasValidity};

const CLASS_PATH : &str = "productOfferingPrice";
const PRICE_VERS : &str = "1.0";

/// Constraints
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstraintRef {
    /// Unique Id
    id: String,
    /// HTTP Reference
    href: String,
    /// Name
    name: String,
    /// Version
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
}



/// Product Offering Price Reference
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductOfferingPriceRef {
    /// Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTTP Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Name
    pub name: String,
}

impl From<ProductOfferingPrice> for ProductOfferingPriceRef {
    fn from(pop : ProductOfferingPrice) -> ProductOfferingPriceRef {
        ProductOfferingPriceRef { 
            id: pop.id.clone(), 
            href: pop.href.clone(), 
            name: pop.get_name(),
        }
    }
}

/// Pricing linked to a Product Offering
#[derive(Clone, Default, Debug, Deserialize, HasId, HasLastUpdate, HasName, HasValidity, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductOfferingPrice {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTML Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Versoin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    constraint: Option<Vec<ConstraintRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_bundle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_update: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring_charge_period_length: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring_charge_period_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_of_measure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax: Option<Vec<TaxItem>>,
}

impl ProductOfferingPrice {
    /// Create a new Price Offering Price object
    pub fn new(name :  impl Into<String>) -> ProductOfferingPrice {
        let mut pop = ProductOfferingPrice::create_with_time();
        pop.version = Some(PRICE_VERS.to_string());
        pop.name = Some(name.into());
        pop
    }
}

/// Container for the payload that generated the event
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct ProductOfferingPriceEvent {
    /// Struct that this event relates to
    pub pop: ProductOfferingPrice,
}

impl TMFEvent<ProductOfferingPriceEvent> for ProductOfferingPrice {
    fn event(&self) -> ProductOfferingPriceEvent {
        ProductOfferingPriceEvent {
            pop : self.clone(),
        }
    }
}

impl EventPayload<ProductOfferingPriceEvent> for ProductOfferingPrice {
    type Subject = ProductOfferingPrice;
    type EventType = ProductOfferingPriceEventType;
    fn to_event(&self,event_type : ProductOfferingPriceEventType) -> Event<ProductOfferingPriceEvent,ProductOfferingPriceEventType> {       
        let now = Utc::now();
        let event_time = chrono::DateTime::from_timestamp(now.timestamp(),0).unwrap();
        let desc = format!("{:?} for {}",event_type,self.get_name());
        Event {
            correlation_id: None,
            description: Some(desc),
            domain: Some(ProductOfferingPrice::get_class()),
            event_id: Uuid::new_v4().to_string(),
            field_path: None,
            href: self.href.clone(),
            id: self.id.clone(),
            title: self.name.clone(),
            event_time: event_time.to_string(),
            priority: None,
            time_occurred: None,
            event_type,
            event: self.event(),
        }
    }
}

/// Product Offering Price Event Type
#[derive(Clone,Default,Debug,Deserialize,PartialEq, Serialize)]
pub enum ProductOfferingPriceEventType {
    /// POP Created
    #[default]
    ProductOfferingPriceCreateEvent,
    /// POP Attribute Value Changed
    ProductOfferingPriceAttributeValueChangeEvent,
    /// POP State Changed
    ProductOfferingPriceStateChangeEvent,
    /// POP Deleted
    ProductOfferingPriceDeleteEvent,
}

#[cfg(test)]
mod test {

    use super::*;

    const POP : &str = "APrice";

    #[test]
    fn test_price_new_name() {
        let pop = ProductOfferingPrice::new(POP);

        assert_eq!(pop.name,Some(POP.into()));
    }

    #[test]
    fn test_price_new_version() {
        let pop = ProductOfferingPrice::new(POP);

        assert_eq!(pop.version,Some(PRICE_VERS.into()));    
    }

    #[test]
    fn test_priceref_from_price() {
        let price = ProductOfferingPrice::new(POP);

        let price_ref = ProductOfferingPriceRef::from(price.clone());

        assert_eq!(price.id,price_ref.id);
        assert_eq!(price.href,price_ref.href);
        assert_eq!(price.get_name(),price_ref.name);
    }
}