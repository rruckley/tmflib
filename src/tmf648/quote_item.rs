//! Quote Item Module

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::quote_price::QuotePrice;
use crate::common::attachment::AttachmentRefOrValue;
use crate::common::note::Note;
use crate::common::related_party::RelatedParty;
use crate::common::related_place::RelatedPlaceRefOrValue;
#[cfg(feature = "build-V4")]
use crate::tmf620::product_offering::ProductOffering;
#[cfg(feature = "build-V5")]
use crate::tmf620::product_offering_v5::ProductOffering;
use crate::tmf620::product_specification::ProductSpecificationRef;

use crate::{HasAttachment, HasDescription, HasName};
use tmflib_derive::{HasAttachment, HasDescription};

const QUOTEITEM_DEF_QTY: u16 = 1;

/// Status of product for Quote Item
#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Default, Debug, Deserialize, HasDescription, Serialize)]
pub struct ProductRefOrValue {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTTP Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Product Description (from TMF620)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Is this a bundle (from TMF620)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bundle: Option<bool>,
    /// Is this customer visible (from TMF620)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_customer_visible: Option<bool>,
    /// Product Name
    pub name: Option<String>,
    /// Product serial number (if known)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_serial_number: Option<String>,
    /// Status of product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ProductStatusType>,
    /// Product Specification (TMF620)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_specification: Option<ProductSpecificationRef>,
    /// Site data for this Quote Item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<Vec<RelatedPlaceRefOrValue>>,
    /// Related Parties for this Quote Item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

impl From<ProductOffering> for ProductRefOrValue {
    fn from(value: ProductOffering) -> Self {
        ProductRefOrValue {
            id: value.id.clone(),
            href: value.href.clone(),
            name: Some(value.get_name()),
            description: value.description.clone(),
            product_specification: value.product_specification.clone(),
            is_bundle: value.is_bundle,
            ..Default::default()
        }
    }
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
    pub note: Option<Vec<Note>>,
    /// Related Party
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    /// Product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<ProductRefOrValue>,
    /// Quote Item Pricing
    #[serde(skip_serializing_if = "Option::is_none")]
    quote_item_price: Option<Vec<QuotePrice>>,
}

impl QuoteItem {
    /// Create a new quote item
    pub fn new() -> QuoteItem {
        let id = Uuid::new_v4().to_string();
        QuoteItem {
            id: Some(id),
            quantity: QUOTEITEM_DEF_QTY,
            ..Default::default()
        }
    }

    /// Set the product for this quoteItem
    pub fn product(mut self, product: ProductOffering) -> QuoteItem {
        self.product = Some(ProductRefOrValue::from(product));
        self
    }

    /// Add QuotePrice to this QuoteItem
    pub fn price(&mut self, price: QuotePrice) {
        match self.quote_item_price.as_mut() {
            Some(v) => v.push(price),
            None => self.quote_item_price = Some(vec![price]),
        }
    }

    /// Get the ProductOffering for this QuoteItem
    pub fn get_offer(&self) -> Option<ProductRefOrValue> {
        self.product.clone()
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "build-V4")]
    use crate::tmf620::product_offering::ProductOffering;
    #[cfg(feature = "build-V5")]
    use crate::tmf620::product_offering_v5::ProductOffering;

    use super::*;

    const PRODSTATUSTYPE_JSON: &str = "\"Created\"";
    const OFFER_NAME: &str = "ProductOffering";

    #[test]
    fn test_quote_item_new() {
        let quote_item = QuoteItem::new();

        assert_eq!(quote_item.id.is_some(), true);
        assert_eq!(quote_item.quantity, QUOTEITEM_DEF_QTY);
    }

    #[test]
    fn test_quote_item_price() {
        let price = QuotePrice::new("A Price");
        let mut quote_item = QuoteItem::new();
        quote_item.price(price);

        assert_eq!(quote_item.quote_item_price.is_some(), true);
    }

    #[test]
    fn test_prodstatustype_deserialize() {
        let prodstatustype: ProductStatusType = serde_json::from_str(PRODSTATUSTYPE_JSON).unwrap();

        assert_eq!(prodstatustype, ProductStatusType::Created);
    }

    #[test]
    fn test_quoteitem_getoffer() {
        let po = ProductOffering::new(OFFER_NAME);

        let quote_item = QuoteItem::new().product(po);

        let prodref = quote_item.get_offer();

        assert_eq!(prodref.is_some(), true);
    }
}
