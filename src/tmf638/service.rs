//! Service Module

use uuid::Uuid;
use serde::{Deserialize,Serialize};

use super::MOD_PATH;
use super::LIB_PATH;



const SERVICE_PATH : &str = "service";

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
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    id: Option<String>,
    href: Option<String>,
    name: String,
    state : ServiceStateType,
}

impl Service {
    /// Create a new service object for the inventory
    pub fn new(name : String) -> Service {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,SERVICE_PATH,id);
        Service {
            id:Some(id), 
            href: Some(href), 
            name,
            state : ServiceStateType::Inactive,
        }
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