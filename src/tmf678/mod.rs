//! Customer Bill Management Module

const MOD_PATH : &str = "tmf678/v5";

#[cfg(feature = "tmf678-v4")]
pub mod customer_bill_v4;

#[cfg(feature = "tmf678-v5")]
pub mod customer_bill_v5;