//! Create Risk Assessment Example

#![allow(unused_imports)]

#[cfg(all(feature = "tmf622", feature = "build-V4"))]
use tmflib::tmf622::product_order_v4::{ProductOrder, ProductOrderRef};
#[cfg(all(feature = "tmf622", feature = "build-V5"))]
use tmflib::tmf622::product_order_v5::{ProductOrder, ProductOrderRef};
#[cfg(all(feature = "tmf696", feature = "build-V4"))]
use tmflib::tmf696::characteristic::Characteristic;
#[cfg(all(feature = "tmf696", feature = "build-V4"))]
use tmflib::tmf696::product_order_risk_assessment::ProductOrderRiskAssessment;

fn main() {
    #[cfg(all(feature = "tmf696", feature = "build-V4"))]
    {
        let mut order = ProductOrder::new();
        order.description = Some("A Customer Order".to_string());
        let mut risk = ProductOrderRiskAssessment::new(ProductOrderRef::from(&order));
        let char = Characteristic::new("name", "value");
        risk.replace_characteristic(char);

        dbg!(risk);
    }
}
