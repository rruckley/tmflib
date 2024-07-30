//! Risk Assessment Result Module

use crate::TimePeriod;
use serde::{Deserialize,Serialize};

/// Assessment Result
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskAssessmentResult {
    /// Assessment Score
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score : Option<Vec<RiskScore>>,
    /// Assessment Validity
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for : Option<TimePeriod>,
}

/// Individual Risk Score
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskScore {
    /// Risk categorisation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_name: Option<RiskType>,
    /// Individual Risk Score
    pub score : f32,
}

/// Risk Categorisation
#[derive(Clone,Default,Debug,Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RiskType {
    /// Fraud
    #[default]
    FraudRisk,
    /// Failed payments
    BadPaymentRisk,
    /// Credit Gaming
    CreditGamingRisk,
    /// Identity Risk
    IDConfidenceRisk,
    /// Payment
    PaymentMethodRisk,
}

#[cfg(test)]
mod test {
    use super::{RiskAssessmentResult, RiskScore, RiskType};

    const RISKRESULT_JSON : &str = "{
        \"score\" : [
            {
                \"riskName\" : \"fraudRisk\",
                \"score\" : 12.34
            }
        ]
    }";
    const RISKSCORE_JSON : &str = "{
        \"riskName\" : \"fraudRisk\",
        \"score\" : 12.34
    }";
    const RISKTYPE_JSON : &str = "\"fraudRisk\"";

    #[test]
    fn test_riskresult_deserialize() {
        let riskresult : RiskAssessmentResult = serde_json::from_str(RISKRESULT_JSON).unwrap();
    }
    #[test]
    fn test_riskscore_deserialize() {
        let riskscore : RiskScore = serde_json::from_str(RISKSCORE_JSON).unwrap();

        assert_eq!(riskscore.risk_name.is_some(),true);
        assert_eq!(riskscore.risk_name.unwrap(),RiskType::FraudRisk);
        assert_eq!(riskscore.score,12.34);
    }
    #[test]
    fn test_risktype_deserialize() {
        let risktype : RiskType = serde_json::from_str(RISKTYPE_JSON).unwrap(); 

        assert_eq!(risktype, RiskType::FraudRisk);
    }
}