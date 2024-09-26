//! Service Test Specification Module

use serde::{Deserialize, Serialize};

use crate::{
    HasId,
    HasName,
    HasDescription,
    HasLastUpdate,
    Uri,
    LIB_PATH,
    TimePeriod
};
use tmflib_derive::{
    HasId,
    HasName,
    HasDescription,
    HasLastUpdate,
};

use super::MOD_PATH;
const CLASS_PATH: &str = "specification";

/// Threshold Consequence
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct MetricDefMeasureConsequence {
    /// Consequence Name
    pub name : Option<String>,
    /// Consequence Description
    pub description : Option<String>,
}

/// Threshold Rule
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct MetricDefMeasureThreasholdRule {
    /// Rule Description
    pub description : Option<String>,
    /// Rule Consequences
    consequence : Vec<MetricDefMeasureConsequence>
}

/// Test Measure Definition
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestMeasureDefinition {
    /// Measure Definition Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name : Option<String>,
    /// Capture Frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_frequency : Option<String>,
    /// Capture Method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method : Option<String>,
    /// Capture Period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_period : Option<String>,
    /// Metric Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_description: Option<String>,
    /// Metric HREF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_href : Option<String>,
    /// Metric Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name : Option<String>,
    /// Unit of Measure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measure : Option<String>,
    /// Validity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for : Option<TimePeriod>,
    /// Value Type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type : Option<String>,
    /// Threhold Rules
    pub threshold_rule : Option<Vec<MetricDefMeasureThreasholdRule>>,
}

/// Service Test Specification
#[derive(Clone,Debug,Default,Deserialize,HasId,HasName,HasDescription,HasLastUpdate,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceTestSpecification {
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description : Option<String>,
    /// Is this part of a bundle?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bundle : Option<bool>,
    /// Last Update Time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update: Option<String>,
    /// Lifecycle Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_status: Option<String>,
    /// Unique Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HREF to specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version : Option<String>,
    /// Test Measures
    pub test_measure_definition : Option<Vec<TestMeasureDefinition>>,
}

#[cfg(test)]
mod test {
    use super::*;

    const SPECIFICATION_ID : &str = "STS123";
    const SPECIFICATION_NAME : &str = "SpecificationName";

    const SPECIFICATION_JSON : &str = "{
        \"id\" : \"STS123\",
        \"name\" : \"SpecificationName\"
    }";

    #[test]
    fn test_specification_deserialize() {
        let specification : ServiceTestSpecification = serde_json::from_str(SPECIFICATION_JSON).unwrap();

        assert_eq!(specification.get_id().as_str(),"STS123");
        assert_eq!(specification.get_name().as_str(),"SpecificationName");
    }

    #[test]
    fn test_specification_hasid() {
        let mut specification = ServiceTestSpecification::default();
        specification.set_id(SPECIFICATION_ID);

        assert_eq!(specification.get_id().as_str(),SPECIFICATION_ID);
    }

    #[test]
    fn test_specification_hasname() {
        let mut specification = ServiceTestSpecification::default();

        specification.set_name(SPECIFICATION_NAME);

        assert_eq!(specification.get_name().as_str(),SPECIFICATION_NAME);
    }
}