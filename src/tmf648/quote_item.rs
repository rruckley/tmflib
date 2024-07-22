//! Quote Item Module

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::attachment::AttachmentRefOrValue;
use crate::common::note::Note;
use crate::common::related_party::RelatedParty;
use crate::common::related_place::RelatedPlaceRefOrValue;
#[cfg(feature = "tmf620-v4")]
use crate::tmf620::product_offering::ProductOffering;
#[cfg(feature = "tmf620-v5")]
use crate::tmf620::product_offering_v5::ProductOffering;

use crate::tmf620::product_specification::ProductSpecificationRef;

use super::quote_price::QuotePrice;

use crate::HasAttachment;
use tmflib_derive::HasAttachment;

/// Status of product for Quote Item
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub enum ProductStatusType {
    /// Created
    #[default]
    Created,
    /// Wait for Active
    PendingActive,
    /// Cancelled
    Cancelled,
    /// Active
    Active,
    /// Wait for terminate
    PendingTerminate,
    /// Terminated
    Terminated,
    /// Suspended
    Suspended,
    /// Aborted
    Aborted,
}

// Not sure if this should be housed in TMF620 but sample payload shows it being local to QuoteItem
/// Quote Item Product 
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ProductRefOrValue {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id : Option<String>,
    /// HTTP Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href : Option<String>,
    /// Product Description (from TMF620)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description : Option<String>,
    /// Is this a bundle (from TMF620)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bundle: Option<bool>,
    /// Is this customer visible (from TMF620)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_customer_visible : Option<bool>,
    /// Product Name
    pub name : String,
    /// Product serial number (if known)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_serial_number : Option<String>,
    /// Status of product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status : Option<ProductStatusType>,
    /// Product Specification (TMF620)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_specification : Option<ProductSpecificationRef>,
    /// Site data for this Quote Item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place : Option<Vec<RelatedPlaceRefOrValue>>,
    /// Related Parties for this Quote Item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

/// Quote Item, line item for a product quote
#[derive(Clone, Default, Debug, Deserialize, HasAttachment, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteItem {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<String>,
    /// Quantity
    pub quantity: u16,
    /// Child Quote Items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_item: Option<Vec<QuoteItem>>,
    /// Attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<AttachmentRefOrValue>>,
    /// Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note : Option<Vec<Note>>,
    /// Related Party
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party : Option<Vec<RelatedParty>>,
    /// Product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product : Option<ProductRefOrValue>,
    /// Quote Item Pricing
    #[serde(skip_serializing_if = "Option::is_none")]
    quote_item_price : Option<Vec<QuotePrice>>,
}

impl QuoteItem {
    /// Create a new quote item
    pub fn new() -> QuoteItem {
        let id = Uuid::new_v4().to_string();
        QuoteItem {
            id : Some(id),
            quantity : 1,
            quote_item_price : Some(vec![]),
            ..Default::default()
        }
    }

    /// Add QuotePrice to this QuoteItem
    pub fn price(&mut self, price : QuotePrice) {
        self.quote_item_price.as_mut().unwrap().push(price);
    }

    /// Get the ProductOffering for this QuoteItem
    pub fn get_offer(&self) -> Option<ProductOffering> {
        match self.product.as_ref() {
            Some(_p) => {
                // p.product_specification;
                None
            },
            None => None,
        }
    }
}
