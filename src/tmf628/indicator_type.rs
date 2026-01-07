use serde::{Deserialize, Serialize};
///This is enumeration for Indicator Type
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IndicatorType {
    /// Indicator type is string
    #[serde(rename = "string")]
    String,
    /// Indicator type is boolean
    #[serde(rename = "int")]
    Int,
    /// Indicator type is boolean
    #[serde(rename = "float")]
    Float,
    /// Indicator type is double
    #[serde(rename = "double")]
    Double,
}
