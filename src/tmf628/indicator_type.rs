use serde::{Serialize, Deserialize};
///This is enumeration for Indicator Type
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IndicatorType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "double")]
    Double,
}
