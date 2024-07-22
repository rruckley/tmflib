//! TMF697 Work Order Management
//! # Versions
//! - V4 Supported
//! - V5 Supported

#[cfg(feature = "tmf697-v4")]
pub mod v4;

#[cfg(feature = "tmf697-v5")]
pub mod v5;

