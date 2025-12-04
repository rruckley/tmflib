use serde::{Deserialize, Serialize};
///This is enumeration for CollectionType state
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CollectionType {
    ///Cumulative collection type
    #[serde(rename = "cumulative")]
    Cumulative,
    ///Delta collection type
    #[serde(rename = "delta")]
    Delta,
    ///Discrete event collection type
    #[serde(rename = "discrete_event")]
    DiscreteEvent,
    ///Gauge collection type
    #[serde(rename = "gauge")]
    Gauge,
    ///Status inspection collection type
    #[serde(rename = "status_inspection")]
    StatusInspection,
}
