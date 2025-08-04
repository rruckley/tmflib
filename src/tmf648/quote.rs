//! Quote Module
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::quote_item::QuoteItem;
use super::quote_price::QuotePrice;
use super::MOD_PATH;
use crate::common::event::{Event, EventPayload};
use crate::common::note::Note;
use crate::common::related_party::RelatedParty;
use crate::common::tmf_error::TMFError;
use crate::tmf651::agreement::AgreementRef;
use crate::{
    DateTime, HasDescription, HasId, HasName, HasNote, HasRelatedParty, HasValidity, TMFEvent,
    TimePeriod, LIB_PATH,
};
use tmflib_derive::{HasDescription, HasId, HasNote, HasRelatedParty, HasValidity};

const CLASS_PATH: &str = "quote";
const QUOTE_VERS: &str = "1.0";

/// Status of the quote object
#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum QuoteStateType {
    /// Quote has been rejected
    Rejected,
    /// Quote status is pending an outcome
    Pending,
    /// Quote is being processed
    InProgress,
    /// Quote has been cancelled
    Cancelled,
    /// Quote is fully approved
    Approved,
    #[default]
    /// Quote has been recieved and accepted
    Accepted,
}

/// Product Quote
#[derive(
    Clone,
    Default,
    Debug,
    Deserialize,
    HasId,
    HasValidity,
    HasNote,
    HasDescription,
    HasRelatedParty,
    Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// URI Reference to quote
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Quote Category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Quote description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Effective Quote Completion Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_quote_completion_date: Option<DateTime>,
    /// Expected Fulfillment Start Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_fulfillment_start_date: Option<DateTime>,
    /// Expected Quote Completion Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_quote_completion_date: Option<DateTime>,
    /// External reference
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Instanct Sync?
    pub instant_sync_quote: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Requested Completion Date
    pub requested_quote_completion_date: Option<DateTime>,
    /// Current quote version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Quote status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<QuoteStateType>,
    /// Validity Period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,

    // External entities
    //
    /// Associated agreement for this quote
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement: Option<Vec<AgreementRef>>,
    /// Notes for Quote
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Note>>,
    /// Vector of quote items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_item: Option<Vec<QuoteItem>>,

    /// Order Submission Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_date: Option<DateTime>,
    /// Requested Start Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime>,
    /// Total Quote Pricing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_total_price: Option<Vec<QuotePrice>>,
    /// Related Parties for this Quote
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

impl Quote {
    /// Create a new Product Quote
    pub fn new() -> Quote {
        let mut quote = Quote::create();
        quote.version = Some(QUOTE_VERS.to_string());
        quote.state = Some(QuoteStateType::Accepted);
        quote
    }

    /// Set external Id for this quote
    pub fn with_external_id(&mut self, id: String) {
        self.external_id = Some(id);
    }

    /// Add a quote item into a product quote
    pub fn add_quote_item(&mut self, item: QuoteItem) -> Result<String, TMFError> {
        match self.quote_item.as_mut() {
            Some(v) => {
                v.push(item);
                Ok(String::from("Quote Item Added"))
            }
            None => {
                self.quote_item = Some(vec![item]);
                Ok(String::from("Vector created and quote Item Added"))
            }
        }
    }

    /// Add a price entry to this quote
    pub fn price(&mut self, price: QuotePrice) {
        match self.quote_total_price.as_mut() {
            Some(v) => v.push(price),
            None => self.quote_total_price = Some(vec![price]),
        }
    }

    /// Get a description for this quote
    pub fn description(&self) -> String {
        match &self.description {
            Some(d) => d.clone(),
            None => {
                format!("Quote-{}", self.get_id())
            }
        }
    }
}

impl HasName for Quote {
    fn find(&self, pattern: &str) -> bool {
        self.description.as_ref().unwrap() == pattern
    }
    fn get_name(&self) -> String {
        self.description
            .as_ref()
            .unwrap_or(&String::from("INVALID"))
            .clone()
    }
    fn set_name(&mut self, name: impl Into<String>) {
        self.description = Some(name.into())
    }

    fn name(mut self, name: impl Into<String>) -> Self {
        self.set_name(name);
        self
    }
}

/// Container for the Quote
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuoteEvent {
    /// Struct that this event relates to
    pub quote: Quote,
}

/// Types of Quote Events
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum QuoteEventType {
    /// Quote created
    QuoteCreateEvent,
    /// Quote changed State
    QuoteStateChangeEvent,
    /// Quote deleted
    QuoteDeleteEvent,
    /// Quote requires further information
    QuoteInformationRequiredEvent,
    /// Quote attribute changed values
    QuoteAttributeValueChangeEvent,
}

impl TMFEvent<QuoteEvent> for Quote {
    fn event(&self) -> QuoteEvent {
        QuoteEvent {
            quote: self.clone(),
        }
    }
}

impl EventPayload<QuoteEvent> for Quote {
    type Subject = Quote;
    type EventType = QuoteEventType;
    fn to_event(
        &self,
        event_type: Self::EventType,
    ) -> crate::common::event::Event<QuoteEvent, Self::EventType> {
        let now = Utc::now();
        let desc = format!(
            "{:?} for {} [{}]",
            event_type,
            self.get_name(),
            self.get_id()
        );
        let event_time = chrono::DateTime::from_timestamp(now.timestamp(), 0).unwrap();

        Event {
            correlation_id: None,
            description: Some(desc),
            domain: Some(Quote::get_class()),
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
    use super::Quote;
    use super::QuoteEventType;
    use super::QuoteStateType;
    use crate::common::event::EventPayload;
    use crate::tmf648::quote::QUOTE_VERS;
    use crate::tmf648::quote_item::QuoteItem;
    use crate::tmf648::quote_price::QuotePrice;
    use crate::{HasId, HasName};

    const QUOTE_EXTERNAL: &str = "ExternalId";
    const QUOTE_PRICE: &str = "QuotePrice";
    #[test]
    fn quote_test_new_vers() {
        let quote = Quote::new();

        assert_eq!(quote.version, Some(QUOTE_VERS.to_string()));
    }

    #[test]
    fn quote_test_new_state() {
        let quote = Quote::new();

        assert_eq!(quote.state, Some(QuoteStateType::Accepted));
    }

    #[test]
    fn quote_test_description() {
        let mut quote = Quote::new();
        quote.description = Some("description".to_string());

        assert_eq!(quote.description(), "description".to_string())
    }

    #[test]
    fn quote_test_no_description() {
        let quote = Quote::new();

        assert_eq!(quote.description(), format!("Quote-{}", quote.get_id()));
    }

    #[test]
    fn test_quote_external_id() {
        let mut quote = Quote::new();

        quote.with_external_id(QUOTE_EXTERNAL.to_string());

        assert_eq!(quote.external_id, Some(QUOTE_EXTERNAL.to_string()));
    }

    #[test]
    fn test_quote_event() {
        let quote = Quote::new();

        let event = quote.to_event(QuoteEventType::QuoteCreateEvent);

        // assert_eq!(quote.description,event.description);
        assert_eq!(event.href, quote.href);
        assert_eq!(event.id, quote.id);
        assert_eq!(event.title.unwrap(), quote.get_name());
    }

    #[test]
    fn test_quote_add_item() {
        let item1 = QuoteItem::new();
        let mut quote = Quote::new();

        let res1 = quote.add_quote_item(item1);

        assert_eq!(res1.is_ok(), true);

        let item2 = QuoteItem::new();

        let res2 = quote.add_quote_item(item2);

        assert_eq!(res2.is_ok(), true);

        assert_eq!(quote.quote_item.is_some(), true);
        assert_eq!(quote.quote_item.unwrap().len(), 2);
    }

    #[test]
    fn test_quote_add_price() {
        let mut quote = Quote::new();
        let price1 = QuotePrice::new(QUOTE_PRICE);
        quote.price(price1);

        let price2 = QuotePrice::new(QUOTE_PRICE);
        quote.price(price2);

        assert_eq!(quote.quote_total_price.is_some(), true);
        assert_eq!(quote.quote_total_price.unwrap().len(), 2);
    }
}
