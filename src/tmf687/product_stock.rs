//! Product Stock Module

use super::MOD_PATH;
use crate::common::event::{Event, EventPayload};
use crate::common::product::ProductRefOrValue;
use crate::common::related_party::RelatedParty;
use crate::common::related_place::RelatedPlaceRefOrValue;
use crate::common::tmf_error::TMFError;
use crate::{DateTime, HasId, HasLastUpdate, HasName, HasRelatedParty, TMFEvent, Uri, };
use tmflib_derive::{HasId, HasLastUpdate, HasName, HasRelatedParty};

// External
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const CLASS_PATH: &str = "productStock";

/// Product Stock Relationship
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ProductStockRelationship {
    relationship_type: String,
}

/// Product Stock Reference
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ProductStockRef {
    href: Uri,
    id: String,
    name: String,
}

impl From<ProductStock> for ProductStockRef {
    fn from(value: ProductStock) -> Self {
        ProductStockRef {
            id: value.get_id(),
            href: value.get_href(),
            name: value.get_name(),
        }
    }
}

/// Product Stock Record
#[derive(
    Clone, Default, Debug, Deserialize, HasId, HasName, HasLastUpdate, HasRelatedParty, Serialize,
)]
pub struct ProductStock {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<Uri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_update: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_party: Option<Vec<RelatedParty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    place: Option<Vec<RelatedPlaceRefOrValue>>,
    stocked_product: ProductRefOrValue,
    product_stock_relationship: Option<Vec<ProductStockRelationship>>,
}

impl ProductStock {
    /// Create a new ProductStock instance
    pub fn new(name: impl Into<String>) -> ProductStock {
        ProductStock {
            name: Some(name.into()),
            ..ProductStock::create()
        }
    }
}

/// Product Stock Class
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum ProductStockEventType {
    /// Default Event
    #[default]
    /// Product Stock Event
    ProductStockCreateEvent,
    /// Product Stock Attribute Value Change Event
    ProductStockAttributeValueChangeEvent,
    /// Product Stock State Change Event
    ProductStockStateChangeEvent,
    /// Product Stock Batch Event
    ProductStockBatchEvent,
    /// Adjust Product Stock Create Event
    ProductStockDeleteEvent,
    /// Adjust Product Stock Attribute Value Change Event
    AdjustProductStockCreateEvent,
    /// Adjust Product Stock Attribute Value Change Event
    AdjustProductStockAttributeValueChangeEvent,
    /// Adjust Product Stock State Change Event
    AdjustProductStockStateChangeEvent,
    /// Adjust Product Stock Batch Event
    AdjustProductStockBatchEvent,
    /// Check Product Stock Create Event
    CheckProductStockCreateEvent,
    /// Check Product Stock Attribute Value Change Event
    CheckProductStockAttributeValueChangeEvent,
    /// Check Product Stock State Change Event
    CheckProductStockStateChangeEvent,
    /// Check Product Stock Batch Event
    CheckProductStockBatchEvent,
    /// Reserve Product Stock Create Event
    ReserveProductStockCreateEvent,
    /// Reserve Product Stock Attribute Value Change Event
    ReserveProductStockAttributeValueChangeEvent,
    /// Reserve Product Stock State Change Event
    ReserveProductStockStateChangeEvent,
    /// Reserve Product Stock Batch Event
    ReserveProductStockBatchEvent,
    /// Query Product Stock Create Event
    QueryProductStockCreateEvent,
    /// Query Product Stock Attribute Value Change Event
    QueryProductStockAttributeValueChangeEvent,
    /// Query Product Stock State Change Event
    QueryProductStockStateChangeEvent,
    /// Query Product Stock Batch Event
    QueryProductStockBatchEvent,
}

// Events
/// Product Stock Event
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ProductStockEvent {
    /// Product Stock
    pub product_stock: ProductStock,
}

impl TMFEvent<ProductStockEvent> for ProductStock {
    fn event(&self) -> ProductStockEvent {
        ProductStockEvent {
            product_stock: self.clone(),
        }
    }
}

impl EventPayload<ProductStockEvent> for ProductStock {
    type Subject = ProductStock;
    type EventType = ProductStockEvent;
    fn to_event(&self, event_type: Self::EventType) -> Event<ProductStockEvent, Self::EventType> {
        let now = Utc::now();
        let event_time = chrono::DateTime::from_timestamp(now.timestamp(), 0).unwrap();
        let desc = format!("{:?} for {}", event_type, self.get_name());
        Event {
            description: Some(desc),
            domain: Some(ProductStock::get_class()),
            event_id: Uuid::new_v4().to_string(),
            href: self.href.clone(),
            id: self.id.clone(),
            title: self.name.clone(),
            event_time: event_time.to_string(),
            event_type,
            event: self.event(),
            ..Event::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const STOCK_NAME: &str = "ProductStock";

    #[test]
    fn test_product_stock_new() {
        let stock = ProductStock::new(STOCK_NAME);

        assert_eq!(stock.name.is_some(), true);
        assert_eq!(stock.get_name().as_str(), STOCK_NAME);
    }

    #[test]
    fn test_productstockref_from_productstock() {
        let stock = ProductStock::new(STOCK_NAME);
        let stock_ref = ProductStockRef::from(stock.clone());

        assert_eq!(stock_ref.id, stock.get_id());
        assert_eq!(stock_ref.href, stock.get_href());
        assert_eq!(stock_ref.name, stock.get_name());
    }
}
