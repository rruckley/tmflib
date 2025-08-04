//! External Identifier Module

use crate::{HasId, Uri, LIB_PATH};
use serde::{Deserialize, Serialize};
use tmflib_derive::HasId;

const CLASS_PATH: &str = "external";
use super::MOD_PATH;

/// External Identifier Reference
#[derive(Clone, Debug, Default, Deserialize, HasId, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalIdentifier {
    external_identifier_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<Uri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    owner: String,
}

#[cfg(test)]
mod test {
    use super::ExternalIdentifier;
    use crate::HasId;

    const EXTERNAL_JSON: &str = "{
        \"externalIdentifierType\" : \"email\",
        \"owner\" : \"customer\"
    }";

    #[test]
    fn test_exteranl_hasid() {
        let mut external = ExternalIdentifier::default();
        external.generate_id();

        let id = external.get_id();

        assert_eq!(external.id, Some(id));
    }

    #[test]
    fn test_external_deserialize() {
        let external: ExternalIdentifier =
            serde_json::from_str(EXTERNAL_JSON).expect("Could not parse EXTERNAL_JSON");

        assert_eq!(external.external_identifier_type.as_str(), "email");
        assert_eq!(external.owner.as_str(), "customer");
    }
}
