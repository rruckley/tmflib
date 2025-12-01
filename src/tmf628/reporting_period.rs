use serde::{Serialize, Deserialize};
///Possible values for the reporting period
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReportingPeriod {
    ///The reporting period is 1 min
    #[serde(rename = "r_1mn")]
    R1Mn,
    ///The reporting period is 5 mins
    #[serde(rename = "r_5mn")]
    R5Mn,
    ///The reporting period is 15 mins
    #[serde(rename = "r_15mn")]
    R15Mn,
    ///The reporting period is 30 mins
    #[serde(rename = "r_30mn")]
    R30Mn,
    ///The reporting period is 1 hr
    #[serde(rename = "r_1h")]
    R1H,
    ///The reporting period is 24 hrs
    #[serde(rename = "r_24h")]
    R24H,
    ///The reporting period is not setr
    #[serde(rename = "na")]
    Na,
}
