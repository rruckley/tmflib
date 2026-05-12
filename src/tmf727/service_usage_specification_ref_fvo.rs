use serde::{Deserialize, Serialize};
///UsageSpecification reference. UsageSpecification is a detailed description of a service usage event that are of interest to the business. It is comprised of characteristics, which define all attributes known for a particular type of usage.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceUsageSpecificationRefFvo {}
impl std::fmt::Display for ServiceUsageSpecificationRefFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
