use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProtocolTransferDataFvo {
    #[serde(rename = "transportProtocol")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<String>,
}
impl std::fmt::Display for ProtocolTransferDataFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
