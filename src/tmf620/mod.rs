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

//! TMF620 Product Catalogue Management
//!
//! Structs associated with product catalogue management

use super::LIB_PATH;
use serde::{Deserialize, Serialize};

#[cfg(feature = "tmf620-v4")]
const MOD_PATH: &str = "productCatalogManagement/v4";
#[cfg(feature = "tmf620-v5")]
const MOD_PATH: &str = "productCatalogManagement/v5";

pub mod bundled_product_offering;
pub mod catalog;
pub mod category;

#[cfg(feature = "v4")]
pub mod product_offering;
#[cfg(feature = "v5")]
pub mod product_offering_v5;
pub mod product_offering_price;
pub mod product_specification;

/// Channel Reference
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChannelRef {}

/// Market Segment Refefence
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketSegmentRef {}

/// Place Reference
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlaceRef {}

/// Service Level Agreement Reference
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SLARef {}

