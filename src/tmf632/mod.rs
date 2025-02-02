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

//! TMF632 Party Management Management
//! # Description
//! Manages data related to Parties either [`individual_v4::Individual`] or [`organization_v4::Organization`]

use serde::{Deserialize,Serialize};

#[cfg(all(feature = "tmf632",feature = "build-V4"))]
const MOD_PATH : &str = "partyManagement/v4";
#[cfg(all(feature = "tmf632",feature = "build-V5"))]
const MOD_PATH : &str = "partyManagement/v5";

#[cfg(all(feature = "tmf632",feature = "build-V4"))]
pub mod individual_v4;
#[cfg(all(feature = "tmf632",feature = "build-V4"))]
pub mod organization_v4;

#[cfg(all(feature = "tmf632",feature = "build-V5"))]
pub mod individual_v5;
#[cfg(all(feature = "tmf632",feature = "build-V5"))]
pub mod organization_v5;

/// General Party characteristic
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Characteristic {
    name: String,
    name_type : String,
    value: String,
    base_type: Option<String>,
    schema_location: Option<String>,
    #[serde(rename = "@type")]
    r#type: Option<String>,
}

#[cfg(test)]
mod test {

    use super::Characteristic;

    const CHAR_JSON : &str = "{
        \"name\" : \"name\",
        \"nameType\" : \"NameType\",
        \"value\" : \"CharName\"
    }";

    #[test]
    fn test_characteristic_deserialise() {
        let char : Characteristic = serde_json::from_str(CHAR_JSON)
            .expect("Could not parse CHAR_JSON");

        assert_eq!(char.name.as_str(),"name");
        assert_eq!(char.name_type.as_str(),"NameType");
        assert_eq!(char.value.as_str(),"CharName");
    }
}