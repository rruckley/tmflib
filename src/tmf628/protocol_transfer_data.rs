use serde::{Serialize, Deserialize};

///Protocol Transfer Data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProtocolTransferData {
    ///Protocol used for the transfer
    #[serde(rename = "transportProtocol")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<String>,
}
impl std::fmt::Display for ProtocolTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
