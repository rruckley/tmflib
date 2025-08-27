//! Agreement Specification Module

use crate::{DateTime, HasId, HasLastUpdate, HasName, HasValidity, TimePeriod, };
use serde::{Deserialize, Serialize};
use tmflib_derive::{HasId, HasLastUpdate, HasName, HasValidity};

use super::MOD_PATH;
const CLASS_PATH: &str = "specification";

/// Agreement Specification
#[derive(
    Clone, Debug, Default, Deserialize, HasId, HasName, HasLastUpdate, HasValidity, Serialize,
)]
pub struct AgreementSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_bundle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_update: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
}

/// Reference to external specification
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct AgreementSpecificationRef {
    description: String,
    href: String,
    id: String,
    name: String,
}

impl From<AgreementSpecification> for AgreementSpecificationRef {
    fn from(value: AgreementSpecification) -> Self {
        AgreementSpecificationRef {
            description: value.description.as_ref().unwrap().clone(),
            id: value.get_id(),
            href: value.get_href(),
            name: value.get_name(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{HasId, HasName};

    use super::{AgreementSpecification, AgreementSpecificationRef};

    const AGREESPEC_ID: &str = "AS123";
    const AGREESPEC_NAME: &str = "AgreementSpecificationName";

    const AGREESPECREF_JSON: &str = "{
        \"description\" : \"Description\",
        \"href\" : \"http://example.com/tmf651/specification/AS123\",
        \"id\" : \"AS123\",
        \"name\" : \"AgreementSpecificationName\"
    }";
    #[test]
    fn test_agreespec_deseralize() {}

    #[test]
    fn test_agreespec_hasid() {}

    #[test]
    fn test_agreespec_hasname() {}

    #[test]
    fn test_agreespec_haslastupdate() {}

    #[test]
    fn test_agreespec_hasvalidity() {}

    #[test]
    fn test_agreespecref_deserialize() {
        let agreespecref: AgreementSpecificationRef =
            serde_json::from_str(AGREESPECREF_JSON).unwrap();

        assert_eq!(agreespecref.description.as_str(), "Description");
        assert_eq!(agreespecref.id.as_str(), "AS123");
        assert_eq!(agreespecref.name.as_str(), "AgreementSpecificationName")
    }

    #[test]
    fn test_agreespecref_from_agreespec() {
        let mut agreespec = AgreementSpecification::default();
        agreespec.set_id(AGREESPEC_ID);
        agreespec.set_name(AGREESPEC_NAME);
        agreespec.description = Some(String::from("Description"));

        let agreespecref = AgreementSpecificationRef::from(agreespec.clone());

        assert_eq!(agreespec.get_id().as_str(), agreespecref.id);
        assert_eq!(agreespec.get_name().as_str(), agreespecref.name);
        assert_eq!(agreespec.description.is_some(), true);
        assert_eq!(
            agreespec.description.unwrap().as_str(),
            agreespecref.description
        );
    }
}
