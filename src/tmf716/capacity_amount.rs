use serde::{Deserialize, Serialize};
///Quantity that defines the Capacity.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CapacityAmount {
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
    ///A value and units that define the `CapacityAmount`, such as 10000 ea, 10B Mb. Instance values are mutually exclusive with From and To capacityAmounts and range interval.
    #[serde(rename = "capacityAmount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity_amount: Option<String>,
    ///The low range value that a Capacity Amount can take on.
    #[serde(rename = "capacityAmountFrom")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity_amount_from: Option<String>,
    ///The upper range value that `CapacityAmount` can take on.
    #[serde(rename = "capacityAmountTo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity_amount_to: Option<String>,
    /**An indicator that specifies the inclusion or exclusion of the capacityAmount From and capacityAmountTo attributes.
    Possible values are "open", "closed", "closedBottom" and "closedTop".*/
    #[serde(rename = "rangeInterval")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_interval: Option<String>,
}
impl std::fmt::Display for CapacityAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
