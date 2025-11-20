use serde::{Serialize, Deserialize};
///Enumeration of supported packing/packaging. All extensions allowed.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PackingEnumType {
    ///No packing
    #[serde(rename = "NO_PACKING")]
    NoPacking,
    ///ZIP packing
    #[serde(rename = "GZIP")]
    Gzip,
    ///TAR packing
    #[serde(rename = "TAR")]
    Tar,
    ///Vendor specific extension packing
    #[serde(rename = "VENDOR_EXT")]
    VendorExt,
    ///Minor extension packing
    #[serde(rename = "MINOR_EXT")]
    MinorExt,
}
