//! TMF697 Work Order Management
//! # Versions
//! - V4 Supported
//! - V5 Supported

#[cfg(all(feature = "tmf697", feature = "build-V4"))]
pub mod v4;

#[cfg(all(feature = "tmf697", feature = "build-V5"))]
pub mod v5;

