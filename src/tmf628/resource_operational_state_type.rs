use serde::{Serialize, Deserialize};
///ResourceOperationalStateType enumerations; values defined by ITU X.731: 'disable': The resource is totally inoperable and unable to provide service to the user(s); 'enable': The resource is partially or fully operable and available for use.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResourceOperationalStateType {
    #[serde(rename = "enable")]
    Enable,
    #[serde(rename = "disable")]
    Disable,
}
