//!
//! Product Offering Module

use crate::common::attachment::AttachmentRefOrValue;
use crate::tmf620::bundled_product_offering::BundledProductOffering;
use crate::tmf620::category::CategoryRef;
use crate::tmf620::product_specification::{
    ProductSpecification, ProductSpecificationCharacteristicValueUse, ProductSpecificationRef,
};

use crate::{HasLastUpdate, HasId, HasName, TimePeriod};
use tmflib_derive::{HasId,HasName,HasLastUpdate};
use crate::tmf634::resource_candidate::ResourceCandidateRef;
use crate::tmf633::service_candidate::ServiceCandidateRef;
use super::product_offering_price::ProductOfferingPriceRef;
use serde::{Deserialize, Serialize};

use super::{ChannelRef,MarketSegmentRef,PlaceRef,SLARef};
use crate::tmf651::agreement::AgreementRef;

use super::LIB_PATH;
use super::MOD_PATH;

const PO_VERS_INIT: &str = "1.0";
const CLASS_PATH: &str = "productOffering";

/// Product Offering Reference
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProductOfferingRef {
    id: String,
    href: String,
    name : String,
}

impl From<ProductOffering> for ProductOfferingRef {
    /// Convert from ProductOffering into ProductOfferingRef
    fn from(po : ProductOffering) -> ProductOfferingRef {
        ProductOfferingRef { 
            id: po.get_id(), 
            href: po.get_href(), 
            name: po.get_name() 
        }
    }
}

/// Product Offering Term
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductOfferingTerm {}

/// Product Offering Relationship
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductOfferingRelationship {
    id: Option<String>,
    href: Option<String>,
    name: Option<String>,
    /// Type of relationship between product offerings
    /// # Example
    /// Parent/Child
    pub relationship_type: Option<String>,
    /// Role of this relationship
    /// # Example
    /// Child
    pub role: Option<String>,
    /// How long is this relationship valid for?
    pub valid_for: Option<TimePeriod>,
}

impl From<ProductOffering> for ProductOfferingRelationship {
    fn from(po : ProductOffering) -> ProductOfferingRelationship {
        ProductOfferingRelationship {
            id: po.id.clone(),
            href: po.href.clone(),
            name: po.name.clone(),
            relationship_type: None,
            role : None,
            valid_for: None,
        }
    }
}

/// Product Offering
#[derive(Clone, Default, Debug, Deserialize, HasId, HasName,HasLastUpdate, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductOffering {
    /// Unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HREF for API use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Description of offering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Does this represent a bundle?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bundle: Option<bool>,
    /// Is this sellable?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sellable: Option<bool>,
    /// When was this last updated?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update: Option<String>,
    /// Current status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_status: Option<String>,
    /// Name of this offering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Status Reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// Version of this offering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Validity Period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    /// Associated agreements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement: Option<Vec<AgreementRef>>,
    /// Attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<AttachmentRefOrValue>>,
    /// Bundled Product Offerings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundled_product_offering: Option<Vec<BundledProductOffering>>,
    /// Categories
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<CategoryRef>>,
    /// Channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Vec<ChannelRef>>,
    /// Market Segments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_segment: Option<Vec<MarketSegmentRef>>,
    /// Places
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<Vec<PlaceRef>>,
    /// Product Offering Price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_offering_price: Option<Vec<ProductOfferingPriceRef>>,
    /// Product Offering Relationship.
    /// Links to other product offers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_offering_relationship: Option<Vec<ProductOfferingRelationship>>,
    /// Product Offering Term
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_offering_term: Option<Vec<ProductOfferingTerm>>,
    /// Product Specification Characteristic Value Use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prod_spec_char_value_use: Option<Vec<ProductSpecificationCharacteristicValueUse>>,
    /// Product Specification
    pub product_specification: Option<ProductSpecificationRef>,
    /// Resource Canididates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_candidate: Option<ResourceCandidateRef>,
    /// Service Candidates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_candidate: Option<ServiceCandidateRef>,
    /// Service Level Agreements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_level_agreement: Option<SLARef>,
}

impl ProductOffering {
    /// Create a new instance of ProductOffering object
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::product_offering_v5::ProductOffering;
    /// let po = ProductOffering::new(String::from("MyOffer"));
    /// ```
    pub fn new(name: impl Into<String>) -> ProductOffering {
        let mut offer = ProductOffering::create_with_time();
        offer.name = Some(name.into());
        offer.version = Some(PO_VERS_INIT.to_string());
        offer.product_offering_relationship = Some(vec![]);
        offer.prod_spec_char_value_use = Some(vec![]);

        offer
    }

    /// Set status of this ProductOffering
    pub fn status(&mut self, status : &str) {
        self.lifecycle_status = Some(status.to_owned());
    }

    /// Added category refernce to ProductOffering
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::product_offering_v5::ProductOffering;
    /// # use tmflib::tmf620::category::{Category,CategoryRef};
    /// let po = ProductOffering::new(String::from("MyOffer"));
    /// let cat= Category::new(String::from("MyCategory"));
    /// let result = po.with_category(CategoryRef::from(&cat));
    /// ```
    pub fn with_category(mut self, category: Category) -> ProductOffering {
        let cat_ref = CategoryRef::from(category);
        self.category = upsert(self.category,category);
        match self.category.as_mut() {
            Some(v) => v.push(cat_ref),
            None => self.category = Some(vec![cat_ref]),
        }
        self
    }

    pub fn upsert<T : IntoIterator,U : HasId>( &i : Some(T), item : U) -> Option<T> {
        match i.as_mut() {
            Some(v) => v.push(U),
            None => Some(vec![item]),
        }
    }

    /// Add characteristic value uses into this Product Offering
    pub fn with_char_value_use(mut self, char_value_use : ProductSpecificationCharacteristicValueUse) -> ProductOffering {
        self.prod_spec_char_value_use.as_mut().unwrap().push(char_value_use);
        self
    }

    /// Create a link between two ProductOfferings
    pub fn link_po(&mut self, remote_po : ProductOffering, relationship_type : &str, role : &str) {
        // Create a link from ourselves into remote_po using type and role prodived.
        let mut offer_rel = ProductOfferingRelationship::from(remote_po);
        offer_rel.relationship_type = Some(relationship_type.to_string());
        offer_rel.role = Some(role.to_string());
        self.product_offering_relationship.as_mut().unwrap().push(offer_rel);
    }
}

#[cfg(test)]
mod test {
    use super::ProductOffering;
    use super::PO_VERS_INIT;

    #[test]
    fn test_po_new_name() {
        let po = ProductOffering::new(String::from("MyOffer"));

        assert_eq!(po.name, Some(String::from("MyOffer")));
    }

    #[test]
    fn test_po_new_version() {
        let po = ProductOffering::new(String::from("MyOffer"));

        assert_eq!(po.version, Some(PO_VERS_INIT.to_string()));
    }
}
