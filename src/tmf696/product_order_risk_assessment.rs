//! Product Order Risk Assessment Mobule

// This should just be relatedplace, not orvalue.
use crate::common::related_place::RelatedPlaceRefOrValue;
use super::{characteristic::Characteristic, risk_assessment_result::RiskAssessmentResult};
#[cfg(feature = "tmf622-v4")]
use crate::tmf622::product_order_v4::ProductOrderRef;
#[cfg(feature = "tmf622-v5")]
use crate::tmf622::product_order_v5::ProductOrderRef;
use crate::{
    HasId,
    Uri,
    LIB_PATH,
};
use tmflib_derive::HasId;
use serde::{Deserialize,Serialize};
use super::MOD_PATH;

/// Class path for Product Order RA
pub const CLASS_PATH : &str = "productOrderRiskAssessment";

/// Product Order Risk Assessment
#[derive(Clone,Default,Debug,Deserialize,HasId,Serialize)]
pub struct ProductOrderRiskAssessment {
    /// Link to Risk Assessment
    pub href : Option<Uri>,
    /// Unique Id
    pub id : Option<String>,
    /// Status of Risk Assessment
    pub status : Option<String>,
    /// Related Place
    pub place : Option<RelatedPlaceRefOrValue>,
    /// Assessment Results
    pub risk_assessment_result: Option<RiskAssessmentResult>,
    /// Product Order Reference
    pub product_order: ProductOrderRef,
    /// Characteristics
    pub characteristic: Option<Vec<Characteristic>>,
}

impl ProductOrderRiskAssessment {
    /// Create a new instance of Product Order Risk Assessment
    pub fn new( order: ProductOrderRef) -> ProductOrderRiskAssessment {
        ProductOrderRiskAssessment {
            product_order : order.clone(),
            ..ProductOrderRiskAssessment::create()
        }
    }

    /// Replaces a characteristic as follows:
    /// - Create characteristic vec[] if required
    /// - Find existing characteristic by name if exists
    /// - Replace found characteristic if found else add new.
    /// - Return found characteristic
    ///
    pub fn replace_characteristic(&mut self, characteristic : Characteristic) -> Option<Characteristic> {
        match &self.characteristic {
            Some(v) => {
                match v.iter().find(|c| { c.name == characteristic.name}) {
                    Some(i) => {
                        let out = i.clone();
                        Some(out)
                    },
                    None => {
                        None
                    }
                }
            },
            None => {
                self.characteristic = Some(vec![characteristic]);
                None
            }
        }
    }
}