//! Product Information Module

use serde::{Deserialize,Serialize};
#[cfg(feature = "tmf620-v4")]
use crate::tmf620::product_offering::ProductOfferingRef;
#[cfg(feature = "tmf620-v5")]
use crate::tmf620::product_offering_v5::ProductOfferingRef;
use crate::tmf620::product_specification::ProductSpecificationRef;
use crate::tmf666::billing_account::BillingAccountRef;
use super::related_place::RelatedPlaceRefOrValue;
use super::related_party::RelatedParty;
use crate::HasDescription;
use tmflib_derive::HasDescription;

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

    // Referenced types
    /// Product Specification (TMF620)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_specification : Option<ProductSpecificationRef>,
    /// Site data for this Quote Item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place : Option<Vec<RelatedPlaceRefOrValue>>,
    /// Related Parties for this Quote Item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    /// Billing account for this product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_account: Option<BillingAccountRef>,
    product_offering : Option<ProductOfferingRef>,
}

#[cfg(test)]
mod test {
    use super::*;

    const PROD_STATUS_TYPE_JSON : &str = "\"Created\"";

    const PRODREF_JSON : &str = "{
        \"id\" : \"PR123\",
        \"name\" : \"ProductRef\",
        \"isBundle\" : false
    }";

    #[test]
    fn test_product_deserialise() {
        let prod : ProductRefOrValue = serde_json::from_str(PRODREF_JSON).unwrap();

        assert_eq!(prod.id.is_some(),true);
        assert_eq!(prod.name.as_str(),"ProductRef");
        assert_eq!(prod.is_bundle.is_some(),true);
        assert_eq!(prod.is_bundle.unwrap(),false);
    }

    #[test]
    fn test_productstatustype_deserialise() {
        let prod_status : ProductStatusType = serde_json::from_str(PROD_STATUS_TYPE_JSON).unwrap();

        assert_eq!(prod_status,ProductStatusType::Created);
    }
}