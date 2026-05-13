use serde::{Deserialize, Serialize};
///`ResourceUsageSpecification` reference. `ResourceUsageSpecification` is a detailed description of a usage event that are of interest to the business and can have charges applied to it. It is comprised of characteristics, which define all attributes known for a particular type of usage.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceUsageSpecificationRefFvo {}
impl std::fmt::Display for ResourceUsageSpecificationRefFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
