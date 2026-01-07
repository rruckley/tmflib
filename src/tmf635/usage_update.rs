use super::{
    RatedProductUsage, RelatedParty, UsageCharacteristic, UsageSpecificationRef, UsageStatusType,
};
use serde::{Deserialize, Serialize};
/**An occurrence of employing a Product, Service, or Resource for its intended purpose, which is of interest to the business and can have charges applied to it. It is comprised of characteristics, which represent attributes of usage.
Skipped properties: id,href*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageUpdate {
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
    ///Description of usage
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Rated product usage
    #[serde(rename = "ratedProductUsage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rated_product_usage: Option<Vec<RatedProductUsage>>,
    /// Related party
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    ///Possible values for the status of the Usage
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<UsageStatusType>,
    /// Usage characteristics
    #[serde(rename = "usageCharacteristic")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_characteristic: Option<Vec<UsageCharacteristic>>,
    ///Date of usage
    #[serde(rename = "usageDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_date: Option<crate::DateTime>,
    ///UsageSpecification reference. UsageSpecification is a detailed description of a usage event that are of interest to the business and can have charges applied to it. It is comprised of characteristics, which define all attributes known for a particular type of usage.
    #[serde(rename = "usageSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_specification: Option<UsageSpecificationRef>,
    ///Type of usage
    #[serde(rename = "usageType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
}
impl std::fmt::Display for UsageUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
