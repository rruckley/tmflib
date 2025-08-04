//! User Role Module

use crate::{HasId, Uri, LIB_PATH};
use serde::{Deserialize, Serialize};
use tmflib_derive::HasId;

use super::MOD_PATH;

const CLASS_PATH: &str = "role";

/// Entitlement for this role
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Entitlement {
    action: String,
    function: String,
    id: String,
}

/// User Role
#[derive(Clone, Debug, Default, Deserialize, HasId, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRole {
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<Uri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    involvement_role: String,
    // Referenced structures
    entitlement: Vec<Entitlement>,
}

#[cfg(test)]
mod test {
    use super::*;

    const ENTITLEMENT_JSON: &str = "{
        \"action\" : \"add\",
        \"function\" : \"more\",
        \"id\" : \"E123\"
    }";

    const USER_ROLE_JSON: &str = "{
        \"involvementRole\" : \"Role\",
        \"entitlement\" : []
    }";

    const USER_ROLE_ID: &str = "UR123";

    #[test]
    fn test_entitlement_deserialize() {
        let entitlement: Entitlement = serde_json::from_str(ENTITLEMENT_JSON).unwrap();

        assert_eq!(entitlement.action.as_str(), "add");
        assert_eq!(entitlement.function.as_str(), "more");
        assert_eq!(entitlement.id.as_str(), "E123");
    }

    #[test]
    fn test_userrole_deserialize() {
        let userrole: UserRole = serde_json::from_str(USER_ROLE_JSON).unwrap();

        assert_eq!(userrole.involvement_role.as_str(), "Role");
    }

    #[test]
    fn test_userrole_hasid() {
        let mut userrole = UserRole::default();

        userrole.set_id(USER_ROLE_ID);

        assert_eq!(userrole.id.is_some(), true);
        assert_eq!(userrole.id.unwrap().as_str(), USER_ROLE_ID);
    }
}
