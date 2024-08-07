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
#[serde(rename_all = "camelCase")]
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

#[cfg(test)]
mod test {
    use crate::tmf696::characteristic::Characteristic;
    use crate::HasId;
    use crate::tmf622::product_order_v4::{ProductOrder, ProductOrderRef};

    use super::ProductOrderRiskAssessment;

    const PORA_JSON : &str = "{
        \"id\" : \"PORA123\",
        \"status\" : \"New\",
        \"productOrder\" : {
            \"id\" : \"PO123\",
            \"href\" : \"http://example.com/tmf622/order/PO123\",
            \"name\" : \"ProductOrderName\"
        }
    }";
    #[test]
    fn test_pora_deseralize() {
        let pora : ProductOrderRiskAssessment = serde_json::from_str(PORA_JSON).unwrap();

        assert_eq!(pora.id.is_some(),true);
        assert_eq!(pora.get_id().as_str(),"PORA123");
        assert_eq!(pora.status.is_some(),true);
    }

    #[test]
    fn test_pora_new() {
        let order = ProductOrderRef::from(ProductOrder::new());
        let pora = ProductOrderRiskAssessment::new(order.clone());

        assert_eq!(pora.product_order.id,order.id);
    }

    #[test]
    fn test_pora_replacechar() {
        let char1 = Characteristic::new("Char", "Value1");
        let char2 = Characteristic::new("Char", "Value2");
        
        let order = ProductOrderRef::from(ProductOrder::new());
        let mut pora = ProductOrderRiskAssessment::new(order.clone());

        // Add char in new
        pora.replace_characteristic(char1);

        assert_eq!(pora.characteristic.is_some(),true);
        assert_eq!(pora.characteristic.clone().unwrap().len(),1);

        pora.replace_characteristic(char2);

        assert_eq!(pora.characteristic.unwrap().len(),1);
    }

}