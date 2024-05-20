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

//! TMF674 Geographic Site Module

#[cfg(feature = "tmf674-v4")]
pub mod geographic_site_v4;
#[cfg(feature = "tmf674-v5")]
pub mod geographic_site_v5;

#[cfg(feature = "tmf674-v4")]
const MOD_PATH : &str = "tmf674/v4";
#[cfg(feature = "tmf674-v5")]
const MOD_PATH : &str = "tmf674/v5";
