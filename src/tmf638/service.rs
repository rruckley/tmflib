//! Service Module

use serde::{Deserialize,Serialize};

use super::MOD_PATH;
use crate::LIB_PATH;

use crate::{HasId,CreateTMF};
use tmflib_derive::HasId;

const CLASS_PATH : &str = "service";

#[derive(Clone,Debug,Default,Deserialize,PartialEq,Serialize)]
#[serde(rename_all = "camelCase")]
enum ServiceStateType {
    FeasibilityChecked,
    Designed,
    Reserved,
    #[default]
    Inactive,
    Active,
    Terminated,
}

/// Service record from the Service Inventory
#[derive(Clone,Debug,Default,Deserialize, HasId, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    id: Option<String>,
    href: Option<String>,
    name: String,
    state : ServiceStateType,
}

impl Service {
    /// Create a new service object for the inventory
    pub fn new(name : impl Into<String>) -> Service {
        let mut service = Service::create();
        service.name = name.into();
        service
    }
}

#[cfg(test)]
mod test {
    use crate::tmf638::service::ServiceStateType;

    use super::Service;
    #[test]
    fn test_service_create_name() {
        let service = Service::new(String::from("AService"));

        assert_eq!(service.name,String::from("AService"));
    }

    #[test]
    fn test_service_default_state() {
        let service = Service::default();

        assert_eq!(service.state , ServiceStateType::Inactive);
    }
}