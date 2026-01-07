use serde::{Serialize, Deserialize};
///Reference of an usage consumption report
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageConsumptionReportRef {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///The type of the referred entity
    #[serde(rename = "@referredType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referred_type: Option<String>,
    ///A URI to a JSON-Schema file that defines additional attributes and relationships
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///When sub-classing, this defines the sub-class Extensible name
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///Free short text describing the usage consumption report content
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Date and time when the usage consumption report was calculated and generated
    #[serde(rename = "effectiveDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<crate::DateTime>,
    ///Hyperlink to access the usage consumption report
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///Unique identifier of the usage consumption report given by the server
    pub id: String,
    ///Usage consumption report name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for UsageConsumptionReportRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
