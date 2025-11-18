use serde::{Serialize, Deserialize};
///Possible values for the reporting period
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReportingPeriod {
    #[serde(rename = "r_1mn")]
    R1Mn,
    #[serde(rename = "r_5mn")]
    R5Mn,
    #[serde(rename = "r_15mn")]
    R15Mn,
    #[serde(rename = "r_30mn")]
    R30Mn,
    #[serde(rename = "r_1h")]
    R1H,
    #[serde(rename = "r_24h")]
    R24H,
    #[serde(rename = "na")]
    Na,
}
