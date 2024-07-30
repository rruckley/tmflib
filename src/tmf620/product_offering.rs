//!
//! Product Offering Module

use crate::common::attachment::AttachmentRefOrValue;
use crate::tmf620::bundled_product_offering::BundledProductOffering;
use crate::tmf620::category::CategoryRef;
use crate::tmf620::product_specification::{
    ProductSpecification, ProductSpecificationCharacteristicValueUse, ProductSpecificationRef,
};
use crate::tmf634::resource_candidate::ResourceCandidateRef;
use crate::tmf633::service_candidate::ServiceCandidateRef;


use crate::{
    HasAttachment,
    HasLastUpdate, 
    HasId, 
    HasName, 
    HasValidity, 
    TimePeriod, 
    DateTime,
    Uri,
    LIB_PATH,
};

use super::product_offering_price::ProductOfferingPriceRef;
use serde::{Deserialize, Serialize};

use super::{ChannelRef,MarketSegmentRef,PlaceRef,SLARef};
use crate::tmf651::agreement::AgreementRef;

use tmflib_derive::{
    HasId,
    HasAttachment,
    HasLastUpdate,
    HasName,
    HasValidity
};

use super::MOD_PATH;

const PO_VERS_INIT: &str = "1.0";
const CLASS_PATH: &str = "productOffering";

/// Product Offering Reference
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProductOfferingRef {
    /// Unique Id
    pub id: String,
    /// HTTP URI
    pub href: String,
    /// Name of offer
    pub name : String,
}

impl From<ProductOffering> for ProductOfferingRef {
    /// Convert from ProductOffering into ProductOfferingRef
    fn from(po : ProductOffering) -> ProductOfferingRef {
        ProductOfferingRef { 
            id: po.id.unwrap_or("MISSING".to_string()).clone(), 
            href: po.href.unwrap_or("MISSING".to_string()).clone(), 
            name: po.name.unwrap_or("MISSING".to_string()).clone() 
        }
    }
}

/// Product Offering Term
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductOfferingTerm {}

/// Product Offering Relationship
#[derive(Clone, Debug, Deserialize, Serialize, HasValidity)]
#[serde(rename_all = "camelCase")]
pub struct ProductOfferingRelationship {
    /// Unique Id
    pub id: Option<String>,
    /// HTTP Uri
    pub href: Option<String>,
    /// Name of referenced Product Offer
    pub name: Option<String>,
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
#[derive(Clone, Default, Debug, Deserialize, HasId, HasAttachment, HasLastUpdate, HasName, HasValidity, Serialize)]
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
    pub last_update: Option<DateTime>,
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

    // META
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@baseType")]
    base_type : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@schemaLocation")]
    schema_location: Option<Uri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    r#type : Option<String>,
}

impl ProductOffering {
    /// Create a new instance of ProductOffering object
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::product_offering::ProductOffering;
    /// let po = ProductOffering::new(String::from("MyOffer"));
    /// ```
    pub fn new(name: impl Into<String>) -> ProductOffering {
        let mut offer = ProductOffering::create_with_time();
        offer.name = Some(name.into());
        offer.version = Some(PO_VERS_INIT.to_string());
        offer.product_offering_relationship = Some(vec![]);
        offer.prod_spec_char_value_use = Some(vec![]);
        offer.base_type = Some(ProductOffering::get_class());
        offer.r#type = Some(ProductOffering::get_class());
        offer
    }

    /// Set status of this ProductOffering
    pub fn status(&mut self, status : &str) {
        self.lifecycle_status = Some(status.to_owned());
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
    pub fn with_category(mut self, category: CategoryRef) -> ProductOffering {
        if self.category.is_none() {
            self.category = Some(vec![]);
        }
        self.category.as_mut().unwrap().push(category);
        self
    }

    /// Add specification into this Product Offering
    pub fn with_specification(mut self, specification: ProductSpecification) -> ProductOffering {
        self.product_specification = Some(ProductSpecificationRef::from(specification));
        self
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
    use super::*;
    use crate::tmf620::category::{Category,CategoryRef};
    use crate::{HasId,HasName};

    const PO_NAME : &str = "An Offer";
    const PO_STATUS: &str = "A Status";
    const CAT_NAME : &str = "A Category";
    const SPEC_NAME: &str = "A Specification";


    const PRODOFFERREF_JSON : &str = "{
        \"id\" : \"PO123\",
        \"href\" : \"http://example.com/tmf620/offering/PO123\",
        \"name\" : \"ProductOffering\"
    }";

    const PRODOFFERTERM_JSON : &str = "{}";
    const PRODOFFERREL_JSON : &str = "{
        \"id\" : \"POR123\",
        \"name\" : \"ProductOfferRel\",
        \"relationshipType\" : \"Parent/Child\",
        \"role\" : \"child\"
    }";
    #[test]
    fn test_po_new_name() {
        let po = ProductOffering::new(PO_NAME);

        assert_eq!(po.name, Some(String::from(PO_NAME)));
    }

    #[test]
    fn test_po_new_version() {
        let po = ProductOffering::new(PO_NAME);

        assert_eq!(po.version, Some(PO_VERS_INIT.into()));
    }

    #[test]
    fn test_poref_from_po() {
        let po = ProductOffering::new(PO_NAME);
        let po_ref = ProductOfferingRef::from(po.clone());

        assert_eq!(po.get_id(),po_ref.id);
        assert_eq!(po.get_href(),po_ref.href);
        assert_eq!(po.get_name(),po_ref.name);
    }

    #[test]
    fn test_por_from_po() {
        let po = ProductOffering::new(PO_NAME);
        let por = ProductOfferingRelationship::from(po.clone());

        assert_eq!(po.id,por.id);
        assert_eq!(po.href,por.href);
        assert_eq!(po.name,por.name);
        assert_eq!(por.relationship_type.is_none(),true);
        assert_eq!(por.role.is_none(),true);
        assert_eq!(por.valid_for.is_none(),true);
    }

    #[test]
    fn test_po_status() {
        let mut po = ProductOffering::new(PO_NAME);
        po.status(PO_STATUS);

        assert_eq!(po.lifecycle_status.unwrap(),PO_STATUS.to_string());
    }

    #[test]
    fn test_po_with_cat() {
        let cat = Category::new(CAT_NAME);
        let po = ProductOffering::new(PO_NAME)
            .with_category(CategoryRef::from(&cat));

        assert_eq!(po.category.is_some(),true);
    }

    #[test]
    fn test_po_with_spec() {
        let spec = ProductSpecification::new(SPEC_NAME);
        let po = ProductOffering::new(PO_NAME)
            .with_specification(spec);

        assert_eq!(po.product_specification.is_some(),true);
    }

    #[test]
    fn test_po_deserialize() {
        let productofferref : ProductOfferingRef = serde_json::from_str(PRODOFFERREF_JSON).unwrap();

        assert_eq!(productofferref.id.as_str(),"PO123");
        assert_eq!(productofferref.name.as_str(),"ProductOffering");
    }

    #[test]
    fn test_po_term_deserialize() {
        let _offerterm : ProductOfferingTerm = serde_json::from_str(PRODOFFERTERM_JSON).unwrap();
    }

    #[test]
    fn test_po_relationship_deserialize() {
        let offer_rel : ProductOfferingRelationship = serde_json::from_str(PRODOFFERREL_JSON).unwrap();

        assert_eq!(offer_rel.id.is_some(),true);
        assert_eq!(offer_rel.name.is_some(),true);
        assert_eq!(offer_rel.relationship_type.is_some(),true);
        assert_eq!(offer_rel.role.is_some(),true);
    }
}
