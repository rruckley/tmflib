//! Monitor Module

use serde::{Deserialize, Serialize};

use crate::{HasId, Uri};
use tmflib_derive::HasId;

use super::MOD_PATH;

const CLASS_PATH: &str = "monitor";

/// Request
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Request {
    header: HeaderItem,
}

/// Response
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Response {
    header: HeaderItem,
}

/// Header Item
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HeaderItem {
    name: String,
    value: String,
}

/// Resource Function Monitoring
#[derive(Clone, Debug, Default, HasId, Deserialize, Serialize)]
pub struct Monitor {
    /// Unique Id
    pub id: Option<String>,
    /// HTTP URI
    pub href: Option<Uri>,
    /// Source URI
    pub source_href: Option<Uri>,
    /// Status
    pub state: Option<String>,
    /// Request
    pub request: Option<Request>,
    /// Response
    pub response: Option<Response>,
}
