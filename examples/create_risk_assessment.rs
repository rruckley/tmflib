//! Create Risk Assessment Example

use tmflib::tmf696::product_order_risk_assessment::ProductOrderRiskAssessment;
use tmflib::tmf622::product_order_v4::{ProductOrder,ProductOrderRef};

fn main() {
    let mut order = ProductOrder::new();
    order.description = Some("A Customer Order".to_string());
    let risk = ProductOrderRiskAssessment::new(ProductOrderRef::from(&order));

    dbg!(risk);
}