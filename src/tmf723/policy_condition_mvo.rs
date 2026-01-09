use serde::{Serialize, Deserialize};
///A PolicyCondition clause is an aggregation of individual PolicyConditions, and is treated as an atomic object that is aggregated by a PolicyRule. It is represented as a Boolean expression, and defines the necessary state and/or prerequisites that define whether the actions aggregated by that same PolicyRule should be performed. If PolicyCondition is successfully resolved then it must hold value TRUE or FALSE. Non successfull resolution does not contain any boolean value
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyConditionMvo {}
impl std::fmt::Display for PolicyConditionMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
