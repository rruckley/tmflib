//! TMF645 Service Qualification Module

#[cfg(feature = "build-V4")]
const MOD_PATH : &str = "serviceQualificationManagement/v4";
#[cfg(feature = "build-V5")]
const MOD_PATH : &str = "serviceQualificationManagement/v5";

pub mod check_service_qualification;
pub mod query_service_qualification;