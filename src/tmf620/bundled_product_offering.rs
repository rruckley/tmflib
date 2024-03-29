//! Bundled Product Offering Module

const BUNDLE_PATH: &str = "bundle";
#[cfg(feature = "v4")]
use super::product_offering::ProductOffering;
#[cfg(feature = "v5")]
use super::product_offering_v5::ProductOffering;
use super::LIB_PATH;
use super::MOD_PATH;

use serde::{Deserialize, Serialize};

/// Bundled Product Offering details
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BundledProductOffering {
    /// Options for bundled product offerings
    pub bundled_product_offering_option: Option<BundledProductOfferingOption>,
    /// Product offering that is bundled 
    pub offer: ProductOffering,
}

impl BundledProductOffering {
    /// Create new options for BundledProductOffering
    pub fn new(name: impl Into<String>) -> BundledProductOffering {
        let mut offer = ProductOffering::new(name.into());
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
        }
    }
}
