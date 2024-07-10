//! Product Information Module

use serde::{Deserialize,Serialize};
use crate::tmf620::product_specification::ProductSpecificationRef;
use super::related_place::RelatedPlaceRefOrValue;
use super::related_party::RelatedParty;

/// Status of product for Quote Item
#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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