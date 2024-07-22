// Copyright [2024] [Ryan Ruckley]

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! TMF632 Party Management Management
//! # Description
//! Manages data related to Parties either [`Individual`] or [`Organization`]

use serde::{Deserialize,Serialize};

#[cfg(feature = "tmf632-v4")]
const MOD_PATH : &str = "partyManagement/v4";
#[cfg(feature = "tmf632-v5")]
const MOD_PATH : &str = "partyManagement/v5";

#[cfg(feature = "tmf632-v4")]
pub mod individual_v4;
#[cfg(feature = "tmf632-v4")]
pub mod organization_v4;

#[cfg(feature = "tmf632-v5")]
pub mod individual_v5;
#[cfg(feature = "tmf632-v5")]
pub mod organization_v5;

/// General Party characteristic
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct Characteristic {
    name: String,
    name_type : String,
    value: String,
    base_type: Option<String>,
    schema_location: Option<String>,
    #[serde(rename = "@type")]
    r#type: Option<String>,
}