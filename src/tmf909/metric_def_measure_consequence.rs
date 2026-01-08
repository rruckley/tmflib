use serde::{Serialize, Deserialize};
use crate::TimePeriod;
/**A MetricDefMeasureConsequence defines the action (prescribed action or notification) to take when a
MetricDefMeasureThresholdRule is crossed.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricDefMeasureConsequence {
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
    ///A narrative that explains in detail what the consequence is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /**A word, term, or phrase by which a
MetricDefMeasureConsequence is known and distinguished from other MetricDefMeasureConsequences.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /**Recommended remedy for a violated threshold. This could be
the hyperlink to the action.*/
    #[serde(rename = "prescribeAction")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prescribe_action: Option<String>,
    /**An indicator used to specify that a consequence should cease
being applied if a value is in the same range as the previous value or continue being applied if a value is in the same range as the previous value.
If the repeatAction is True, if the consequence is always applied as soon as the MetricMeasure value is in the range of values and if the repeatAction is False, the consequence is applied only if the previous MetricMeasure value was not in the same range.*/
    #[serde(rename = "repeatAction")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repeat_action: Option<bool>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for MetricDefMeasureConsequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
