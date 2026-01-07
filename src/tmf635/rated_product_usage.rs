use super::{Money, ProductRef};
use serde::{Deserialize, Serialize};
///An occurrence of employing a product for its intended purpose with all rating details
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RatedProductUsage {
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
    ///A base / value business entity used to represent money
    #[serde(rename = "bucketValueConvertedInAmount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket_value_converted_in_amount: Option<Money>,
    ///Boolean indicating if usage have been billed or not
    #[serde(rename = "isBilled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_billed: Option<bool>,
    ///Indicates if the rated amount is exempt of tax
    #[serde(rename = "isTaxExempt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_tax_exempt: Option<bool>,
    ///Type of tariff applied
    #[serde(rename = "offerTariffType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer_tariff_type: Option<String>,
    ///Product reference
    #[serde(rename = "productRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_ref: Option<ProductRef>,
    ///Type of amount
    #[serde(rename = "ratingAmountType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rating_amount_type: Option<String>,
    ///Date of usage rating
    #[serde(rename = "ratingDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rating_date: Option<crate::DateTime>,
    ///A base / value business entity used to represent money
    #[serde(rename = "taxExcludedRatingAmount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_excluded_rating_amount: Option<Money>,
    ///A base / value business entity used to represent money
    #[serde(rename = "taxIncludedRatingAmount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_included_rating_amount: Option<Money>,
    ///Tax rate
    #[serde(rename = "taxRate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_rate: Option<f64>,
    /**Tag value: [usage]: the usage is always rated outside a usage bundle
    [included usage]: the usage is rated inside a usage bundle
    [non included usage]: the usage bundle is exhausted. The usage is rated outside the usage bundle*/
    #[serde(rename = "usageRatingTag")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_rating_tag: Option<String>,
}
impl std::fmt::Display for RatedProductUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
