use serde::{Serialize, Deserialize};
use super::EntityRefMvo;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstraintRefMvo {
    #[serde(flatten)]
    pub entity_ref_mvo: EntityRefMvo,
    ///constraint version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for ConstraintRefMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ConstraintRefMvo {
    type Target = EntityRefMvo;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref_mvo
    }
}
impl std::ops::DerefMut for ConstraintRefMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref_mvo
    }
}
