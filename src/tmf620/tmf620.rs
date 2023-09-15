//! TMF620 Product Catalogue Management

/// Agreement Reference
pub struct AgreementRef {}

/// Attachment Reference or Value
pub struct AttachmentRefOrValue {}

/// Bundled Product Offering
pub struct BundledProductOffering {}



/// Category Resource
pub struct Category {
    /// Subcategory
    pub sub_category : Vec<CategoryRef>,
    /// Product Offering
    pub product_offering : Vec<ProductOfferingRef>,
}

/// Category Reference
pub struct CategoryRef {
    id  : String,
    href    : String,
    name    : String,
    version : String,   
}

/// Channel Reference
pub struct ChannelRef {}

/// Market Segment Refefence
pub struct MarketSegmentRef {}

/// Place Reference
pub struct PlaceRef {}

/// Product Offering Reference
pub struct ProductOfferingRef {}

pub struct ProductOfferingPriceRef {}

pub struct ProductOfferingTerm {}

/// Product Specification Reference
pub struct ProductSpecificationRef {}

/// Product Specification Characteristic Value Use
pub struct ProductSpecificationCharacteristicValueUse {}

/// Product Offering
pub struct ProductOffering {
    pub id          : String,
    pub href        : String,
    pub description : String,
    pub is_bundle   : bool,
    pub is_sellable : bool,
    pub last_update : String,
    pub lifecycle_status : String,
    pub name        : String,
    pub status_reason : String,
    pub version     : String,
    pub valid_for   : String,

    pub agreement   : Vec<AgreementRef>,
    /// Attachments
    pub attachment  : Vec<AttachmentRefOrValue>,
    pub bundled_product_offering : Vec<BundledProductOffering>,
    /// Categories
    pub category    : Vec<CategoryRef>,
    pub channel     : Vec<ChannelRef>,
    pub market_segment : Vec<MarketSegmentRef>,
    pub place       : Vec<PlaceRef>,
    pub product_offering_price  : Vec<ProductOfferingPriceRef>,
    pub product_offering_term   : Vec<ProductOfferingTerm>,
    pub prod_sepc_char_value_use : Vec<ProductSpecificationCharacteristicValueUse>,
    pub product_specification   : Vec<ProductSpecificationRef>,
    pub resource_candidate      : Vec<ResourceCandidateRef>,
    pub service_candidate       : Vec<ServiceCandidateRef>,
    pub service_level_agreement : Vec<SLARef>,
}



/// Resource Candidate Reference 
pub struct ResourceCandidateRef {}

/// Service Candidate Reference
pub struct ServiceCandidateRef {}

/// Service Level Agreement Reference 
pub struct SLARef {}