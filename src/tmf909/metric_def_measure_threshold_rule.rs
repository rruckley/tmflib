use serde::{Serialize, Deserialize};
use super::{Duration, MetricDefMeasureConsequence};
/**A MetricDefMeasureThresholdRule is a rule that defines the condition (raise or clear) to achieve to apply
consequences when a threshold is crossed or ceased to be crossed. It also defines the severity of the
raise or clear of the threshold.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricDefMeasureThresholdRule {
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
    /**An operator that when applied on a value specifies whether the
value is the same or not. This operator is used to compare with the conformanceTargetExact if used.*/
    #[serde(rename = "conformanceComparatorExact")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conformance_comparator_exact: Option<bool>,
    /**An operator that when applied on a value specifies whether a
threshold is crossed or ceased to be crossed. This operator is used to Service Test Management API REST Specification compare with the conformanceTargetLower if used.*/
    #[serde(rename = "conformanceComparatorLower")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conformance_comparator_lower: Option<String>,
    /**An operator that when applied on a value specifies whether a
threshold is crossed or ceased to be crossed. This operator is used to compare with the conformanceTargetUpper if used.*/
    #[serde(rename = "conformanceComparatorUpper")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conformance_comparator_upper: Option<String>,
    ///To cater for values that are not numerical test metrics (e.g. a DSL line can be Synchronised or Unsynchronised. If the latter, the test should result in a rule violation). The allowed value can contain a REGEX expression.
    #[serde(rename = "conformanceTargetExact")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conformance_target_exact: Option<String>,
    /**A value used to determine if the threshold is crossed or ceases
to be crossed. It represents the lower limit. The value should be less than the conformanceTargetUpper if used. The conformance comparators should also be logically defined so as to not lead to a logically impossible condition.*/
    #[serde(rename = "conformanceTargetLower")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conformance_target_lower: Option<String>,
    /**A value used to determine if the threshold is crossed or ceases
to be crossed. It represents the Upper limit. The value should be greater than the conformanceTargetLower if used. The conformance comparators should also be logically defined so as to not lead to a logically impossible condition.*/
    #[serde(rename = "conformanceTargetUpper")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conformance_target_upper: Option<String>,
    ///A list of consequences (actions, notifications) that will arise if the threshold is crossed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consequence: Option<Vec<MetricDefMeasureConsequence>>,
    ///Description for the MetricDefMeasureThresholdRule .
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Name for the MetricDefMeasureThresholdRule .
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /**The number of allowed crossing occurrences in reference to the
tolerancePeriod without a consequence being initiated.*/
    #[serde(rename = "numberOfAllowedCrossing")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_allowed_crossing: Option<i64>,
    /**A threshold can be generated in different severity levels. A
crossing for each level may require a different condition and possibly trigger a different consequence.*/
    #[serde(rename = "thresholdRuleSeverity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold_rule_severity: Option<String>,
    ///A time interval in a given unit of time
    #[serde(rename = "tolerancePeriod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerance_period: Option<Duration>,
}
impl std::fmt::Display for MetricDefMeasureThresholdRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
