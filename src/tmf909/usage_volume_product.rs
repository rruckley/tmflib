use serde::{Serialize, Deserialize};
use super::{ConsumptionSummary, NetworkProduct, UsageVolumeBalance};
///A bucket (called UsageVolumeProduct in the SID model) represents a quantity of usage, as 2 hours national calls or 50 sms for example. It could be either a quantity or an amount in a currency (i.e. It could represent a fixed number of SMS, MMS, minutes of calls, quantity of data, number of events as well as a specific amount in a given currency). It requires one or more network products from which usages will debit the bucket.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageVolumeProduct {
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
    ///The balance(s) (called UsageVolumeBalance in the SID model) associated with the bucket
    #[serde(rename = "bucketBalance")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket_balance: Option<Vec<UsageVolumeBalance>>,
    ///The consumption counter(s) (called ConsumptionSummary in the SID model) associated with the bucket
    #[serde(rename = "bucketCounter")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket_counter: Option<Vec<ConsumptionSummary>>,
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///Unique identifier of the bucket
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///True if the bucket is shared between several devices or users
    #[serde(rename = "isShared")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_shared: Option<bool>,
    ///Bucket name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Reference to one or more network products from which usages will debit the bucket
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<Vec<NetworkProduct>>,
    ///Type of usage concerned by the bucket, such as voice, sms, data
    #[serde(rename = "usageType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
}
impl std::fmt::Display for UsageVolumeProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
