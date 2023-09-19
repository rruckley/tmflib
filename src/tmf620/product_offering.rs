//!
//! Product Offering Module

use crate::tmf620::tmf620_catalog_management::{
    AgreementRef,
    BundledProductOffering,
    ChannelRef,
    MarketSegmentRef,
    PlaceRef,
    ResourceCandidateRef,
    ServiceCandidateRef,
    SLARef,
};
use crate::tmf620::category::CategoryRef;
use crate::tmf620::product_specification::{
    ProductSpecificationCharacteristicValueUse,
    ProductSpecificationRef,
};
use crate::common::attachment::AttachmentRefOrValue;
use serde::{Deserialize,Serialize};

/// Product Offering Reference
#[derive(Deserialize, Serialize)]
pub struct ProductOfferingRef {}

/// Product Offering Price Reference
pub struct ProductOfferingPriceRef {}

/// Product Offering Term
pub struct ProductOfferingTerm {}

/// Product Offering
pub struct ProductOffering {
    /// Unique identifier
    pub id          : String,
    /// HREF for API use
    pub href        : String,
    /// Description of offering
    pub description : String,
    /// Does this represent a bundle?
    pub is_bundle   : bool,
    /// Is this sellable?
    pub is_sellable : bool,
    /// When was this last updated?
    pub last_update : String,
    /// Current status
    pub lifecycle_status : String,
    /// Name of this offering
    pub name        : String,
    /// Status Reason
    pub status_reason : String,
    /// Version of this offering
    pub version     : String,
    /// Validity Period
    pub valid_for   : String,

    /// Associated agreements
    pub agreement   : Vec<AgreementRef>,
    /// Attachments
    pub attachment  : Vec<AttachmentRefOrValue>,
    /// Bundled Product Offerings
    pub bundled_product_offering : Vec<BundledProductOffering>,
    /// Categories
    pub category    : Vec<CategoryRef>,
    /// Channels
    pub channel     : Vec<ChannelRef>,
    /// Market Segments
    pub market_segment : Vec<MarketSegmentRef>,
    /// Places
    pub place       : Vec<PlaceRef>,
    /// Product Offering Price
    pub product_offering_price  : Vec<ProductOfferingPriceRef>,
    /// Product Offering Term
    pub product_offering_term   : Vec<ProductOfferingTerm>,
    /// Product Specification Characteristic Value Use
    pub prod_sepc_char_value_use : Vec<ProductSpecificationCharacteristicValueUse>,
    /// Product Specification
    pub product_specification   : Vec<ProductSpecificationRef>,
    /// Resource Canididates
    pub resource_candidate      : Vec<ResourceCandidateRef>,
    /// Service Candidates
    pub service_candidate       : Vec<ServiceCandidateRef>,
    /// Service Level Agreements
    pub service_level_agreement : Vec<SLARef>,
}