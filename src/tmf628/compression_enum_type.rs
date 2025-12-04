use serde::{Deserialize, Serialize};
///Enumeration of supported compressions. All extensions allowed.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CompressionEnumType {
    ///No compression
    #[serde(rename = "no_compression")]
    NoCompression,
    ///GZIP compression
    #[serde(rename = "gzip")]
    Gzip,
    ///TAR compression
    #[serde(rename = "vendor_ext")]
    VendorExt,
    ///Minor extension compression
    #[serde(rename = "minor_ext")]
    MinorExt,
}
