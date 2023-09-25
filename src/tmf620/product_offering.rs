//!
//! Product Offering Module

use crate::tmf620::tmf620_catalog_management::{
    AgreementRef,
    ChannelRef,
    MarketSegmentRef,
    PlaceRef,
    ResourceCandidateRef,
    ServiceCandidateRef,
    SLARef,
};
use crate::tmf620::bundled_product_offering::BundledProductOffering;
use crate::tmf620::category::CategoryRef;
use crate::tmf620::product_specification::{
    ProductSpecificationCharacteristicValueUse,
    ProductSpecificationRef, ProductSpecification,
};
use crate::common::attachment::AttachmentRefOrValue;
use serde::{Deserialize,Serialize};
use chrono::naive::NaiveDateTime;
use chrono::Utc;
use uuid::Uuid;

use super::MOD_PATH;
use super::LIB_PATH;

const PO_VERS_INIT : &str = "1.0";
const PO_PATH : &str = "offer";

/// Product Offering Reference
#[derive(Debug,Deserialize, Serialize)]
pub struct ProductOfferingRef {}

/// Product Offering Price Reference
#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct ProductOfferingPriceRef {}

/// Product Offering Term
#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct ProductOfferingTerm {}

/// Product Offering
#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct ProductOffering {
    /// Unique identifier
    pub id          : Option<String>,
    /// HREF for API use
    pub href        : Option<String>,
    /// Description of offering
    pub description : Option<String>,
    /// Does this represent a bundle?
    pub is_bundle   : bool,
    /// Is this sellable?
    pub is_sellable : bool,
    /// When was this last updated?
    pub last_update : Option<String>,
    /// Current status
    pub lifecycle_status : Option<String>,
    /// Name of this offering
    pub name        : String,
    /// Status Reason
    pub status_reason : Option<String>,
    /// Version of this offering
    pub version     : Option<String>,
    /// Validity Period
    pub valid_for   : Option<String>,

    /// Associated agreements
    pub agreement   : Option<Vec<AgreementRef>>,
    /// Attachments
    pub attachment  : Option<Vec<AttachmentRefOrValue>>,
    /// Bundled Product Offerings
    pub bundled_product_offering : Option<Vec<BundledProductOffering>>,
    /// Categories
    pub category    : Option<Vec<CategoryRef>>,
    /// Channels
    pub channel     : Option<Vec<ChannelRef>>,
    /// Market Segments
    pub market_segment : Option<Vec<MarketSegmentRef>>,
    /// Places
    pub place       : Option<Vec<PlaceRef>>,
    /// Product Offering Price
    pub product_offering_price  : Option<Vec<ProductOfferingPriceRef>>,
    /// Product Offering Term
    pub product_offering_term   : Option<Vec<ProductOfferingTerm>>,
    /// Product Specification Characteristic Value Use
    pub prod_sepc_char_value_use : Option<Vec<ProductSpecificationCharacteristicValueUse>>,
    /// Product Specification
    pub product_specification   : Vec<ProductSpecificationRef>,
    /// Resource Canididates
    pub resource_candidate      : Option<Vec<ResourceCandidateRef>>,
    /// Service Candidates
    pub service_candidate       : Option<Vec<ServiceCandidateRef>>,
    /// Service Level Agreements
    pub service_level_agreement : Option<Vec<SLARef>>,
}

impl ProductOffering {
    /// Create a new instance of ProductOffering object
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::product_offering::ProductOffering;
    /// let po = ProductOffering::new(String::from("MyOffer"));
    /// ```
    pub fn new(name : String) -> ProductOffering {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,PO_PATH,id);
        let now = Utc::now();
        let time = NaiveDateTime::from_timestamp_opt(now.timestamp(), 0).unwrap();
        ProductOffering { 
            id: Some(id), 
            href: Some(href), 
            description: None, 
            is_bundle: false, 
            is_sellable: false, 
            last_update: Some(time.to_string()), 
            lifecycle_status: None, 
            name, 
            status_reason: None, 
            version: Some(PO_VERS_INIT.to_string()), 
            valid_for: None, 
            agreement: None, 
            attachment: None, 
            bundled_product_offering: None, 
            category: None,
            channel: None, 
            market_segment: None, 
            place: None, 
            product_offering_price: None, 
            product_offering_term: None, 
            prod_sepc_char_value_use: None, 
            product_specification: vec![],
            resource_candidate : None,
            service_candidate : None,
            service_level_agreement : None,
        }
    }

    /// Added category refernce to ProductOffering
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::product_offering::ProductOffering;
    /// # use tmflib::tmf620::category::{Category,CategoryRef};
    /// let po = ProductOffering::new(String::from("MyOffer"));
    /// let cat= Category::new(String::from("MyCategory"));
    /// let result = po.with_category(CategoryRef::from(&cat));
    /// ```
    pub fn with_category(mut self, category : CategoryRef) -> ProductOffering {
        if self.category.is_none() {
            self.category = Some(vec![]);
        }
        self.category.as_mut().unwrap().push(category);
        self
    }

    pub fn with_specification(mut self, specification : ProductSpecification) -> ProductOffering {
        self.product_specification.push(ProductSpecificationRef::from(specification));
        self
    }
}

#[cfg(test)]
mod test {
    use super::ProductOffering;
    use super::PO_VERS_INIT;

    #[test]
    fn test_po_new_name() {
        let po = ProductOffering::new(String::from("MyOffer"));

        assert_eq!(po.name , String::from("MyOffer"));
    }

    #[test]
    fn test_po_new_version() {
        let po = ProductOffering::new(String::from("MyOffer"));

        assert_eq!(po.version , Some(PO_VERS_INIT.to_string()));
    }

}