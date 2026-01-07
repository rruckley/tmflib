use serde::{Serialize, Deserialize};
use super::{CustomerBillStateType};
use crate::common::entity::Entity;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer360CustomerBillVo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Bill reference known by the customer or the party and displayed on the bill. Could be different from the id
    #[serde(rename = "billNo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bill_no: Option<String>,
    ///Category of the bill produced : normal, duplicate, interim, last, trial customer or credit note for example
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    ///Recommended Enumeration Type (not formal forced in standard): Valid values for the lifecycle state of the bill: new = 'bill is ready to validate or to sent', validated = 'bill is checked (manual / automatic)', sent = 'bill is sent with the channel defined in the billingaccount', settled = 'bill is payed', partiallySettled = 'bill is partially payed', onHold = 'bill will not be in further processing until open issues connected to the bill are solved'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<CustomerBillStateType>,
}
impl std::fmt::Display for Customer360CustomerBillVo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Customer360CustomerBillVo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Customer360CustomerBillVo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
