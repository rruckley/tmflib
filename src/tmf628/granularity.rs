use serde::{Serialize, Deserialize};
///Sampling rate of the collection or production of performance indicators.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Granularity {
    #[serde(rename = "g_1mn")]
    G1Mn,
    #[serde(rename = "g_5mn")]
    G5Mn,
    #[serde(rename = "g_15mn")]
    G15Mn,
    #[serde(rename = "g_30mn")]
    G30Mn,
    #[serde(rename = "g_1h")]
    G1H,
    #[serde(rename = "g_24h")]
    G24H,
    #[serde(rename = "g_1m")]
    G1M,
    #[serde(rename = "g_1y")]
    G1Y,
    #[serde(rename = "na")]
    Na,
}
