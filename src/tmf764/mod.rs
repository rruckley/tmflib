// Copyright [2025] [Ryan Ruckley]

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! TMF764 Cost Management API

const MOD_PATH: &str = "costManagement/v5";

use serde::{Deserialize, Serialize};

pub mod actual_cost;
pub mod projected_cost;

use crate::Uri;

/// Cost Reference
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CostRef {
    /// Unique identifier
    pub id: String,
    /// Hyperlink reference
    pub href: String,
    /// Name
    pub name: Option<String>,

    /// Base Type this type is derived from if creating sub-classes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@baseType")]
    pub base_type: Option<String>,
    /// Schema Definition of the sub-class (if required)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@schemaLocation")]
    pub schema_location: Option<Uri>,
    /// Name for this Type when sub-classing
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    /// What type is this reference referring to?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@referredType")]
    pub referred_type: Option<String>,
}
