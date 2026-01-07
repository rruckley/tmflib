use serde::{Deserialize, Serialize};
///Set of criteria to be followed by all parties.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PromotionCriteriaMvo {
    ///The base type for use in polymorphic collections
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///A link to the schema describing a resource (for type extension).
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///The class type of the actual resource (for type extension).
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///Logic operator of this criteria instruction: '=' or '>' or '<' or '>=' or '<=' or '<>'.
    #[serde(rename = "criteriaOperator")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria_operator: Option<String>,
    ///The parameter (factor) of the criteria.The basic factors are abstracted from these data sources.There are several detail parameters.
    #[serde(rename = "criteriaParameter")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria_parameter: Option<String>,
    ///The value is filled for the comparison of the criteria.
    #[serde(rename = "criteriaValue")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria_value: Option<String>,
}
impl std::fmt::Display for PromotionCriteriaMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
