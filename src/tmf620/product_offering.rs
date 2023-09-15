//!
//! Product Offering Module

use crate::tmf620::tmf620::{
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

/// Product Offering Reference
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
    pub status_reason : String,
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
    pub prod_sepc_char_value_use : Vec<ProductSpecificationCharacteristicValueUse>,
    pub product_specification   : Vec<ProductSpecificationRef>,
    pub resource_candidate      : Vec<ResourceCandidateRef>,
    pub service_candidate       : Vec<ServiceCandidateRef>,
    pub service_level_agreement : Vec<SLARef>,
}