//! Create Risk Assessment Example

use tmflib::tmf696::product_order_risk_assessment::ProductOrderRiskAssessment;
use tmflib::tmf696::characteristic::Characteristic;
#[cfg(feature = "tmf622-v4")]
use tmflib::tmf622::product_order_v4::{ProductOrder,ProductOrderRef};
#[cfg(feature = "tmf622-v5")]
use tmflib::tmf622::product_order_v5::{ProductOrder,ProductOrderRef};


fn main() {
    let mut order = ProductOrder::new();
    order.description = Some("A Customer Order".to_string());
    let mut risk = ProductOrderRiskAssessment::new(ProductOrderRef::from(&order));
    let char = Characteristic::new("name","value");
    risk.replace_characteristic(char);

    dbg!(risk);
}