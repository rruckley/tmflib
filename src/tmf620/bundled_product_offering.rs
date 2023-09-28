//! Bundled Product Offering Module

const BUNDLE_PATH: &str = "bundle";
use super::product_offering::ProductOffering;
use super::LIB_PATH;
use super::MOD_PATH;

use serde::{Deserialize, Serialize};

/// Bundled Product Offering details
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BundledProductOffering {
    pub bundled_product_offering_option: Option<BundledProductOfferingOption>,
    pub offer: ProductOffering,
}

impl BundledProductOffering {
    /// Create new options for BundledProductOffering
    pub fn new(name: String) -> BundledProductOffering {
        let mut offer = ProductOffering::new(name);
        // Update href to point to bundle instead of standard offer path
        let href = format!(
            "/{}/{}/{}/{}",
            LIB_PATH,
            MOD_PATH,
            BUNDLE_PATH,
            offer.id.as_ref().unwrap()
        );
        offer.href = Some(href.clone());
        BundledProductOffering {
            offer,
            bundled_product_offering_option: None,
        }
    }

    pub fn with_option(mut self, option: BundledProductOfferingOption) -> BundledProductOffering {
        self.bundled_product_offering_option = Some(option);
        self
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BundledProductOfferingOption {
    number_rel_offer_default: u8,
    number_rel_offer_lower_limit: u8,
    number_rel_offer_upper_limit: u8,
}

impl BundledProductOfferingOption {
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
        }
    }
}
