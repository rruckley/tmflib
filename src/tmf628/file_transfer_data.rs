use serde::{Serialize, Deserialize};
use super::{CompressionType, Duration, PackingType, ProtocolTransferData};
use crate::common::extensible::Extensible;

///File Transfer Data schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTransferData {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Protocol Transfer Data schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub protocol_transfer_data: ProtocolTransferData,
    ///File compression type.
    #[serde(rename = "compressionType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<CompressionType>,
    ///File format type.
    #[serde(rename = "fileFormat")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_format: Option<String>,
    ///Location where the file is stored or retrieved from.
    #[serde(rename = "fileLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_location: Option<String>,
    ///Specify if the output file(s) are to be packed.
    #[serde(rename = "packingType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packing_type: Option<PackingType>,
    ///A time interval in a given unit of time
    #[serde(rename = "retentionPeriod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<Duration>,
}
impl std::fmt::Display for FileTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for FileTransferData {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for FileTransferData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
