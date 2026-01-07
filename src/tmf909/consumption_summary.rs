use serde::{Serialize, Deserialize};
use super::{NetworkProductRef, Quantity, RelatedParty};
use crate::TimePeriod;

///The consumption counters (called ConsumptionSummary in the SID model) detail for example the different kind of consumption done on the bucket.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConsumptionSummary {
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
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "consumptionPeriod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consumption_period: Option<TimePeriod>,
    ///Type of the consumption counter. We can give for example a counter of the used value for a bucket (counterType=used for example) or the value of the consumption done out of the bucket(s) (counterType=outOfBucket for example)
    #[serde(rename = "counterType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counter_type: Option<String>,
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///unique identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Counter level. The counter can be given globally for the bucket or detailed by user or by network product for example in case of shared bucket
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    ///Reference of a product
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<NetworkProductRef>,
    ///Related Party reference. A related party defines party or party role linked to a specific entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<RelatedParty>,
    ///An amount in a given unit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Quantity>,
    ///Value of the counter in a formatted string used for display needs for example
    #[serde(rename = "valueName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_name: Option<String>,
}
impl std::fmt::Display for ConsumptionSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
