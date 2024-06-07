//! Shopping Cart Risk Assessment Module

use serde::{Deserialize,Serialize};
use crate::{
    HasId,
    LIB_PATH,
    Uri,
};
use tmflib_derive::HasId;
use super::risk_assessment_result::RiskAssessmentResult;

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
}