use serde::{Serialize, Deserialize};
///PolicyExpression is a constraint based on text expression and parsed by an Expression Language (SpEL, JS, Groovy, FEEL, ...)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyExpressionFvoFvoFvo {
    #[serde(rename = "@type")]
    pub type_: String,
    ///Expression language command
    pub expression: String,
    ///Defines expression language used to build expression
    #[serde(rename = "expressionLanguage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression_language: Option<String>,
}
impl std::fmt::Display for PolicyExpressionFvoFvoFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
