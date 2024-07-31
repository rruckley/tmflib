//! Shopping Cart Risk Assessment Module

use serde::{Deserialize,Serialize};
use crate::{
    HasId,
    LIB_PATH,
    Uri,
};
use tmflib_derive::HasId;
use super::risk_assessment_result::RiskAssessmentResult;
use super::characteristic::Characteristic;

use super::MOD_PATH;

const CLASS_PATH : &str = "shoppingCartRiskAssessment";

/// Shopping Cart Risk Assessment
#[derive(Clone,Default,Debug,HasId,Deserialize,Serialize)]
pub struct ShoppingCartRiskAssessment {
    /// HRef
    pub href : Option<Uri>,
    /// Id
    pub id : Option<String>,
    /// Assessment Results
    pub risk_assessment_result: Option<RiskAssessmentResult>,
    /// Characteristics
    pub characteristic: Option<Vec<Characteristic>>,
}

#[cfg(test)]
mod test {

    use super::*;

    const CART_RISK_ID : &str = "123QH";
    const CARTRISK_JSON : &str = "{
        \"href\" : \"http://example.com/cartrisk/123\",
        \"id\" : \"123\"
    }";
    #[test]
    fn test_cartrisk_deserialize() {
        let cartrisk : ShoppingCartRiskAssessment = serde_json::from_str(CARTRISK_JSON).unwrap();

        assert_eq!(cartrisk.id.is_some(),true);
        assert_eq!(cartrisk.id.unwrap().as_str(),"123");

        assert_eq!(cartrisk.href.is_some(),true);
        assert_eq!(cartrisk.href.unwrap().as_str(),"http://example.com/cartrisk/123");
    }

    #[test]
    fn test_cartrisk_hasid() {
        let mut cartrisk = ShoppingCartRiskAssessment::default();
        cartrisk.set_id(CART_RISK_ID);

        assert_eq!(cartrisk.get_id().as_str(),CART_RISK_ID);    
    }
}