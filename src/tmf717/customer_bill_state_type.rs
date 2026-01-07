use serde::{Serialize, Deserialize};
///Recommended Enumeration Type (not formal forced in standard): Valid values for the lifecycle state of the bill: new = 'bill is ready to validate or to sent', validated = 'bill is checked (manual / automatic)', sent = 'bill is sent with the channel defined in the billingaccount', settled = 'bill is payed', partiallySettled = 'bill is partially payed', onHold = 'bill will not be in further processing until open issues connected to the bill are solved'
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CustomerBillStateType {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "onHold")]
    OnHold,
    #[serde(rename = "validated")]
    Validated,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "settled")]
    Settled,
    #[serde(rename = "partiallyPaid")]
    PartiallyPaid,
}
