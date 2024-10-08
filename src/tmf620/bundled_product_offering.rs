//! Bundled Product Offering Module

const CLASS_PATH: &str = "bundle";
#[cfg(all(feature = "tmf620",feature = "build-V4"))]
use super::product_offering::ProductOffering;
#[cfg(all(feature = "tmf674",feature = "build-V5"))]
use super::product_offering_v5::ProductOffering;

use super::MOD_PATH;
use crate::{
    LIB_PATH,
    HasId,
    HasName,
};
use tmflib_derive::{HasId,HasName};

use serde::{Deserialize, Serialize};

/// Bundled Product Offering details
#[derive(Clone, Default, Debug, HasId, HasName, Deserialize, Serialize)]
pub struct BundledProductOffering {
    /// Unique Id
    pub id : Option<String>,
    /// HTTP URI
    pub href: Option<String>,
    lifecycle_status: Option<String>,
    /// Name of bundle
    pub name: Option<String>,
    /// Options for bundled product offerings
    pub bundled_product_offering_option: Option<BundledProductOfferingOption>,
    /// Product offering that is bundled 
    pub offer: ProductOffering,
}

impl BundledProductOffering {
    /// Create new options for BundledProductOffering
    pub fn new(name: impl Into<String>) -> BundledProductOffering {
        let offer = ProductOffering::new(name.into().clone());
        // Update href to point to bundle instead of standard offer path
        
        BundledProductOffering {
            offer: offer.clone(),
            bundled_product_offering_option: None,
            name : Some(offer.get_name()),
            ..BundledProductOffering::default()
        }
    }

    /// Add option into bundled product offering
    pub fn with_option(mut self, option: BundledProductOfferingOption) -> BundledProductOffering {
        self.bundled_product_offering_option = Some(option);
        self
    }
}

/// Options for bundled product offerings
/// # Detalis
/// This controls the cardinality of included bundled offerings, e.g. min max default etc.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundledProductOfferingOption {
    number_rel_offer_default: u8,
    number_rel_offer_lower_limit: u8,
    number_rel_offer_upper_limit: u8,
}

impl BundledProductOfferingOption {
    /// Create a new Bundled Product Offering Option
    /// # Detalis
    /// This covers the cardinality of included bundeld offerings
    pub fn new(default: u8, min: u8, max: u8) -> BundledProductOfferingOption {
        BundledProductOfferingOption {
            number_rel_offer_default: default,
            number_rel_offer_lower_limit: min,
            number_rel_offer_upper_limit: max,
        }
    }
}

impl From<ProductOffering> for BundledProductOffering {
    fn from(po: ProductOffering) -> BundledProductOffering {
        // Clone source class (po) to leave source untouched
        BundledProductOffering {
            bundled_product_offering_option: None,
            offer: po.clone(),
            ..BundledProductOffering::create()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::HasName;

    use super::*;

    const BPO_NAME: &str = "A Bundle";
    const BPO_DEFAULT: u8 = 1;
    const BPO_MIN: u8 = 0;
    const BPO_MAX: u8 = 100;

    const OFFER_NAME : &str = "ProductOffer";

    const BPO_JSON : &str = "{
        \"id\" : \"BPO123\",
        \"name\" : \"BundleProductOffering\",
        \"offer\" : {}
    }";

    const BPO_OPTION_JSON : &str = "{
        \"numberRelOfferDefault\" : 1,
        \"numberRelOfferLowerLimit\" : 2,
        \"numberRelOfferUpperLimit\" : 3
    }";

    #[test]
    fn test_bpo_new() {
        let bpo = BundledProductOffering::new(BPO_NAME);

        assert_eq!(bpo.get_name(),BPO_NAME.to_string());
    }

    #[test]
    fn test_bpo_with_option() {
        let bpo_option = BundledProductOfferingOption::new(BPO_DEFAULT,BPO_MIN,BPO_MAX);
        let bpo = BundledProductOffering::new(BPO_NAME)
            .with_option(bpo_option);

        assert_eq!(bpo.bundled_product_offering_option.is_some(),true);
    }

    #[test]
    fn test_bpo_option() {
        let bpo_option = BundledProductOfferingOption::new(BPO_DEFAULT,BPO_MIN,BPO_MAX);

        assert_eq!(bpo_option.number_rel_offer_default,BPO_DEFAULT);
        assert_eq!(bpo_option.number_rel_offer_lower_limit,BPO_MIN);
        assert_eq!(bpo_option.number_rel_offer_upper_limit,BPO_MAX);
    }

    #[test]
    fn test_bpo_from_po() {
        let offer = ProductOffering::new(OFFER_NAME);

        let bpo = BundledProductOffering::from(offer.clone());

        assert_eq!(bpo.offer.get_name().as_str(),OFFER_NAME);
    }

    #[test]
    fn test_bpo_deserialize() {
        let bpo : BundledProductOffering = serde_json::from_str(BPO_JSON)
            .expect("Could not parse BPO_JSON");

        assert_eq!(bpo.id.is_some(),true);
        assert_eq!(bpo.get_id(),"BPO123");
        assert_eq!(bpo.name.is_some(),true);
        assert_eq!(bpo.get_name().as_str(),"BundleProductOffering");
    }

    #[test]
    fn test_bpo_option_deserialize() {
        let bpo_option : BundledProductOfferingOption = serde_json::from_str(BPO_OPTION_JSON)
            .expect("Could not parse BPO_OPTION_JSON");

        assert_eq!(bpo_option.number_rel_offer_default,1);
        assert_eq!(bpo_option.number_rel_offer_lower_limit,2);
        assert_eq!(bpo_option.number_rel_offer_upper_limit,3);
    }
}
