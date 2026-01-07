use serde::{Serialize, Deserialize};
use super::{Characteristic, MeasureThresholdRuleViolation};
///A TestMeasure specifies a measure of a specific aspect of a product, service, or resource test, such as lost packets or connectivity status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestMeasure {
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
    ///The number of digits of accuracy captured for associated Metrics
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f64>,
    ///The date and time that the metric was captured
    #[serde(rename = "captureDateTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capture_date_time: Option<crate::DateTime>,
    ///The method used to capture the Metrics (This may be replaced by a set of entities similar to the Performance Monitoring Ref)
    #[serde(rename = "captureMethod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<String>,
    ///Brief description of the metric
    #[serde(rename = "metricDescription")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_description: Option<String>,
    ///Hyperlink to access a metric for detail information
    #[serde(rename = "metricHref")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_href: Option<String>,
    ///The name of the metric
    #[serde(rename = "metricName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    ///A list of rules that were violated in this test measure
    #[serde(rename = "ruleViolation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_violation: Option<Vec<MeasureThresholdRuleViolation>>,
    ///The unit of measure for the metric values, such as meters, cubic yards, kilograms [ISO 1000].
    #[serde(rename = "unitOfMeasure")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
    ///Describes a given characteristic of an object or entity through a name/value pair.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Characteristic>,
}
impl std::fmt::Display for TestMeasure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
