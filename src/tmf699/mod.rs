// Copyright [2026] [Ryan Ruckley]

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! TMF699 Sales Management
//! # Versions
//! - V4 Supported
//! - V5 Supported
//! 

const TMF_MODULE: &str = "salesManagement";

#[cfg(all(feature = "tmf699", feature = "build-V4"))]
pub mod sales_lead_v4;
#[cfg(all(feature = "tmf699", feature = "build-V5"))]
pub mod sales_lead_v5;
#[cfg(all(feature = "tmf699", feature = "build-V5"))]
pub mod sales_opportunity_item_v5;
#[cfg(all(feature = "tmf699", feature = "build-V5"))]
pub mod sales_opportunity_v5;

/// Modules in this API `MOD_PATH`
pub fn get_objects() -> Vec<&'static str> {
    vec![
        #[cfg(feature = "build-V4")]
        sales_lead_v4::CLASS_PATH,
        #[cfg(feature = "build-V5")]
        sales_lead_v5::CLASS_PATH,
        #[cfg(feature = "build-V5")]
        sales_opportunity_v5::CLASS_PATH,
        #[cfg(feature = "build-V5")]
        sales_opportunity_item_v5::CLASS_PATH,
    ]
}
