use serde::{Serialize, Deserialize};
use super::{
    Characteristic, RelatedParty, ServiceRef, ServiceTestSpecificationRef, TestMeasure,
};
use crate::TimePeriod;

/// A service test is an entity that exists for a controlled test invocation on a service. The service
/// test is executed according to a schedule and contains service test configuration parameters that are to be
/// applied at execution time, and service test measures that result.
/// wwSkipped properties: id,href,@type,@schemaLocation,@baseType*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceTestUpdate {
    ///List of characteristics with values that define the test run
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub characteristic: Option<Vec<Characteristic>>,
    ///Description of the service test
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The end date and time of the service test
    #[serde(rename = "endDateTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<crate::DateTime>,
    /**An indication of whether the service test is running in
"PROACTIVE" or "ONDEMAND" mode*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    ///The name of the service test
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Party related to the test
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    ///Service reference, for when Service is used by other entities
    #[serde(rename = "relatedService")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_service: Option<ServiceRef>,
    ///The start date and time of the service test.
    #[serde(rename = "startDateTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<crate::DateTime>,
    ///The actual state the service test is in. Recommended states are found in ExecutionStateType schema possible values include acknowledged, rejected, pending, inProgress, cancelled, completed and failed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    ///The results of the test in terms of the measured metrics
    #[serde(rename = "testMeasure")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_measure: Option<Vec<TestMeasure>>,
    ///The service test specification used by the service test.
    #[serde(rename = "testSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_specification: Option<ServiceTestSpecificationRef>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for ServiceTestUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
