use serde::{Serialize, Deserialize};
/**An Applied Consequence defines the action (prescribed action or notification) to take when a
MeasureThresholdRuleViolation occurs.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppliedConsequence {
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
    /**The action for a violated threshold. This could be a hyperlink to
the action.*/
    #[serde(rename = "appliedAction")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_action: Option<String>,
    ///A narrative that explains in detail what the consequence is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /**A word, term, or phrase by which Consequence is known and
distinguished from other MetricDefMeasureConsequences.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /**An indicator used to specify that a consequence should cease
being applied if a value is in the same range as the previous value or continue being applied if a value is in the same range as the previous value.
If the repeatAction is True, if the consequence is always applied as soon as the MetricMeasure value is in the range of values and if the repeatAction is False, the consequence is applied only if the previous MetricMeasure value was not in the same range.*/
    #[serde(rename = "repeatAction")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repeat_action: Option<bool>,
}
impl std::fmt::Display for AppliedConsequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
