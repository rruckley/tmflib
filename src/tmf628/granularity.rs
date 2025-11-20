use serde::{Serialize, Deserialize};
///Sampling rate of the collection or production of performance indicators.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Granularity {
    ///Sampling rate of 1 minute
    #[serde(rename = "g_1mn")]
    G1Mn,
    ///Sampling rate of 5 minutes
    #[serde(rename = "g_5mn")]
    G5Mn,
    ///Sampling rate of 15 minutes
    #[serde(rename = "g_15mn")]
    G15Mn,
    ///Sampling rate of 30 minutes
    #[serde(rename = "g_30mn")]
    G30Mn,
    ///Sampling rate of 1 hour
    #[serde(rename = "g_1h")]
    G1H,
    ///Sampling rate of 4 hours
    #[serde(rename = "g_24h")]
    G24H,
    ///Sampling rate of 1 day
    #[serde(rename = "g_1m")]
    G1M,
    ///Sampling rate of 1 year
    #[serde(rename = "g_1y")]
    G1Y,
    ///Not applicable
    #[serde(rename = "na")]
    Na,
}
