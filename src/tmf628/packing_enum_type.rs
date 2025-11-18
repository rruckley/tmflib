use serde::{Serialize, Deserialize};
///Enumeration of supported packing/packaging. All extensions allowed.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PackingEnumType {
    #[serde(rename = "NO_PACKING")]
    NoPacking,
    #[serde(rename = "GZIP")]
    Gzip,
    #[serde(rename = "TAR")]
    Tar,
    #[serde(rename = "VENDOR_EXT")]
    VendorExt,
    #[serde(rename = "MINOR_EXT")]
    MinorExt,
}
