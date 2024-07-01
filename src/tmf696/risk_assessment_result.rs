//! Risk Assessment Result Module

use crate::TimePeriod;
use serde::{Deserialize,Serialize};

/// Assessment Result
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct RiskAssessmentResult {
    /// Assessment Score
    pub score : Option<Vec<RiskScore>>,
    /// Assessment Validity
    valid_for : Option<TimePeriod>,
}

/// Individual Risk Score
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct RiskScore {
    /// Risk categorisation
    pub risk_name: Option<RiskType>,
    /// Individual Risk Score
    pub score : f32,
}

/// Risk Categorisation
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
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