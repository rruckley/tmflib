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

//! TMF622 Product Order Management

#[cfg(feature = "build-V4")]
const MOD_PATH: &str = "productOrderingManagement/v4";
#[cfg(feature = "build-V5")]
const MOD_PATH: &str = "productOrderingManagement/v5";

#[cfg(all(feature = "tmf622", feature = "build-V4"))]
pub mod product_order_v4;
#[cfg(all(feature = "tmf622", feature = "build-V5"))]
pub mod product_order_v5;

#[cfg(all(feature = "tmf622", feature = "build-V5"))]
pub mod milestone;
pub mod product_order_item;
