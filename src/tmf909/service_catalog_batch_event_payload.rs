use serde::{Serialize, Deserialize};
use super::ServiceCatalog;
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceCatalogBatchEventPayload {
    /**The root entity for service catalog management.
A service catalog is a group of service specifications made available through service candidates that an organization provides to the consumers (internal consumers like its employees or B2B customers or B2C customers).
A service catalog typically includes name, description and time period that is valid for. It will have a list of ServiceCandidate catalog items. A ServiceCandidate is an entity that makes a ServiceSpecification available to a catalog.
A ServiceCandidate and its associated ServiceSpecification may be "published" - made visible -in any number of ServiceCatalogs, or in none.*/
    #[serde(rename = "serviceCatalog")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_catalog: Option<ServiceCatalog>,
}
impl std::fmt::Display for ServiceCatalogBatchEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
