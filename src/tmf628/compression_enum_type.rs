use serde::{Serialize, Deserialize};
///Enumeration of supported compressions. All extensions allowed.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CompressionEnumType {
    #[serde(rename = "no_compression")]
    NoCompression,
    #[serde(rename = "gzip")]
    Gzip,
    #[serde(rename = "vendor_ext")]
    VendorExt,
    #[serde(rename = "minor_ext")]
    MinorExt,
}
