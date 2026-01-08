use serde::{Serialize, Deserialize};
use super::{RelatedParty, UsageVolumeProduct};
///An usage consumption report enables to know at a given point the balances and the consumption counters related to various buckets (SMS, Voice, Data for example). It could be calculated for a device identified by a public key (msisdn number for a mobile device for example or PSTN or VOIP number for a fix device), for a subscribed offer or option or for an user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageConsumptionReport {
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
    ///Bucket(s) included in the offer or option subscribed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<Vec<UsageVolumeProduct>>,
    ///Free short text describing the usage consumption report content
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Date and time when the usage consumption report was calculated and generated
    #[serde(rename = "effectiveDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<crate::DateTime>,
    ///Usage consumption report name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Related Party reference. A related party defines party or party role linked to a specific entity.
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_party: Option<RelatedParty>,
}
impl std::fmt::Display for UsageConsumptionReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
