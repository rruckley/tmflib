use serde::{Serialize, Deserialize};
use super::{Duration, MetricDefMeasureThresholdRule};
use crate::TimePeriod;

///A TestMeasureDefinition specifies a measure of a specific aspect of a product, service, or resource test, such as lost packets or connectivity status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestMeasureDefinition {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///A URI to a JSON-Schema file that defines additional attributes and relationships
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///When sub-classing, this defines the sub-class Extensible name
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///The frequency of capture for the metric. Note: This may be replaced by a set of entities similar to the Performance Monitoring
    #[serde(rename = "captureFrequency")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capture_frequency: Option<String>,
    ///The method used to capture the Metric. Note: This may be replaced by a set of entities similar to the Performance Monitoring
    #[serde(rename = "captureMethod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<String>,
    ///A time interval in a given unit of time
    #[serde(rename = "capturePeriod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capture_period: Option<Duration>,
    ///Brief description of the metric
    #[serde(rename = "metricDescription")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_description: Option<String>,
    ///Hyperlink to access a metric for detail information
    #[serde(rename = "metricHref")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_href: Option<String>,
    ///The name of a metric that in the test measure
    #[serde(rename = "metricName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    ///The name of the TestMeasureDefinition
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The rule(s) associated with the measure threshold
    #[serde(rename = "thresholdRule")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold_rule: Option<Vec<MetricDefMeasureThresholdRule>>,
    ///Name of a service test specification
    #[serde(rename = "unitOfMeasure")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    ///A kind of value that the Metric value can take on, such as numeric, text, and so forth
    #[serde(rename = "valueType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}
impl std::fmt::Display for TestMeasureDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
