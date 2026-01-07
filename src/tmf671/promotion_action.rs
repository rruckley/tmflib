use serde::{Serialize, Deserialize};
use super::EntityRef;
///Action of the promotion. When the customer meets the conditions in the promotion pattern, the customer can be given the benefits in the action.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PromotionAction {
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
    ///Reference to the action entity such as voucher, promotion code, an existing offering, physical gift etc.
    #[serde(rename = "actionEntityRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_entity_ref: Option<EntityRef>,
    ///Action type can be one of the following: voice (minute), data (MB), data (GB), SMS, bonus point, physical gift, voucher, promotion code, an existing offering, currency.
    #[serde(rename = "actionType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    /**When the Promotion type is 1: Award, it means the amount or value of the awards decided by actionType, such as: Amount of gift, Amount of bonus, Value of discount;
 • When the Promotion type is 2: Discount, it means the value of the discount;
 • When the Promotion type is 3: reduction, it means the value of the reduced money.*/
    #[serde(rename = "actionValue")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_value: Option<String>,
    ///Unique identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl std::fmt::Display for PromotionAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
