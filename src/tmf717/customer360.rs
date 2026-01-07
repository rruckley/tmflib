use serde::{Serialize, Deserialize};
use super::{
    Customer360AccountVo, Customer360AgreementVo, Customer360AppointmentVo,
    Customer360CustomerBillVo, Customer360CustomerVo, Customer360LoyaltyAccountVo,
    Customer360PartyInteractionVo, Customer360PaymentMethodVo, Customer360ProductOrderVo,
    Customer360ProductVo, Customer360PromotionVo, Customer360QuoteVo,
    Customer360RecommendationVo, Customer360ServiceProblemVo, Customer360TroubleTicketVo,
    Entity, TimePeriod,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Customer360 {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///List of accounts of a customer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub account: Vec<Customer360AccountVo>,
    ///List of agreements of a customer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agreement: Vec<Customer360AgreementVo>,
    ///List of appointments of a customer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub appointment: Vec<Customer360AppointmentVo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer360CustomerVo>,
    ///List of bills of a customer.
    #[serde(rename = "customerBill")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub customer_bill: Vec<Customer360CustomerBillVo>,
    ///List of loyalty account of a customer.
    #[serde(rename = "loyaltyAccount")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub loyalty_account: Vec<Customer360LoyaltyAccountVo>,
    ///List of party interactions of a customer.
    #[serde(rename = "partyInteraction")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub party_interaction: Vec<Customer360PartyInteractionVo>,
    ///List of payment methods of a customer.
    #[serde(rename = "paymentMethod")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payment_method: Vec<Customer360PaymentMethodVo>,
    ///List of orders of a customer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub product: Vec<Customer360ProductVo>,
    ///List of product orders of a customer.
    #[serde(rename = "productOrder")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub product_order: Vec<Customer360ProductOrderVo>,
    ///List of promotions of a customer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub promotion: Vec<Customer360PromotionVo>,
    ///List of quoate of a customer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub quote: Vec<Customer360QuoteVo>,
    ///List of recommendations for a customer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommendation: Vec<Customer360RecommendationVo>,
    ///List of service problems of a customer.
    #[serde(rename = "serviceProblem")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_problem: Vec<Customer360ServiceProblemVo>,
    ///List of trouble tickets for a customer.
    #[serde(rename = "troubleTicket")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trouble_ticket: Vec<Customer360TroubleTicketVo>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for Customer360 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Customer360 {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Customer360 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
