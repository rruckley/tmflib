use serde::{Serialize, Deserialize};
use super::{ConsumptionSummary, RelatedParty};
///An instantiated network product (specialization of a product) subscribed by a customer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkProduct {
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
    ///Reference to the network product
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///Unique identifier of the network product
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Network product name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Counters detailing usage consumption out of the buckets
    #[serde(rename = "outOfBucketCounter")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub out_of_bucket_counter: Option<Vec<ConsumptionSummary>>,
    ///Public number associated to the network product (msisdn number for mobile line for example)
    #[serde(rename = "publicIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_identifier: Option<String>,
    ///References of the users of the network product
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<Vec<RelatedParty>>,
}
impl std::fmt::Display for NetworkProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
