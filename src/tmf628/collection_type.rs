use serde::{Serialize, Deserialize};
///This is enumeration for CollectionType state
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CollectionType {
    #[serde(rename = "cumulative")]
    Cumulative,
    #[serde(rename = "delta")]
    Delta,
    #[serde(rename = "discrete_event")]
    DiscreteEvent,
    #[serde(rename = "gauge")]
    Gauge,
    #[serde(rename = "status_inspection")]
    StatusInspection,
}
