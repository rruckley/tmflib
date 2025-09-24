//! Related Entity Module

use serde::{Deserialize, Serialize};

use crate::HasName;
use crate::Uri;

/// Reference to another TMF schema
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct EntityRef {
    /// Entity HREF
    pub href: String,
    /// Entity Id
    pub id : String,
    /// Entity Name
    pub name : String,
}

/// Reference to another TMF schema
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct RelatedEntity {
    /// Referenced Name
    pub name: String,
    /// Referenced Id
    pub id: String,
    /// Referenced HREF
    pub href: Uri,
    /// Referenced Role
    pub role: Option<String>,
    /// Referred Type
    pub referred_type: String,
}

/// Generate a related entity from any TMF object that has implemented HasName trait
impl<T: HasName> From<T> for RelatedEntity {
    fn from(value: T) -> Self {
        RelatedEntity {
            name: value.get_name(),
            id: value.get_id(),
            href: value.get_href(),
            referred_type: T::get_class(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tmf651::agreement::Agreement;
    use crate::{HasId, HasName};

    use super::RelatedEntity;

    const AGREEMENT_NAME: &str = "AnAgreement";

    #[test]
    fn test_relatedentity_from() {
        let agreement = Agreement::new(AGREEMENT_NAME);
        let agree_ref = &agreement;

        let entity = RelatedEntity::from(agreement.clone());

        assert_eq!(entity.name, agree_ref.get_name());
        assert_eq!(entity.id, agree_ref.get_id().as_str());
        assert_eq!(entity.href, agree_ref.get_href().as_str());
        assert_eq!(entity.referred_type, Agreement::get_class());
    }
}
