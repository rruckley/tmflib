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
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ChannelRef {}

/// Market Segment Refefence
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MarketSegmentRef {}

/// Place Reference
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlaceRef {}

/// Service Level Agreement Reference
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SLARef {}

#[cfg(test)]
mod test {
    use super::*;

    const EMPTY_JSON: &str = "{}";

    #[test]
    fn test_channelref_deserialise() {
        let channel = ChannelRef::default();

        let channel_str = serde_json::to_string(&channel);

        assert_eq!(channel_str.is_ok(),true);
    }

    #[test]
    fn test_marketsegment_deserialise() {
        let segment = MarketSegmentRef::default();

        let segment_str = serde_json::to_string(&segment);

        assert_eq!(segment_str.is_ok(),true);  
    }

    #[test]
    fn test_placeref_deserialise() {
        let place = PlaceRef::default();

        let place_str = serde_json::to_string(&place);

        assert_eq!(place_str.is_ok(),true);
    }

    #[test]
    fn test_slaref_deserialise() {
        let sla = SLARef::default();

        let sla_str = serde_json::to_string(&sla);

        assert_eq!(sla_str.is_ok(),true);
    }

    #[test]
    fn test_channelref_deserialize() {
        let _channelref : ChannelRef = serde_json::from_str(EMPTY_JSON).expect("Could not parse Empty JSON");
    }

    #[test]
    fn test_marketsegmentref_deserialize() {
        let _marketsegmentref : MarketSegmentRef  = serde_json::from_str(EMPTY_JSON).expect("Could not parse Emoty JSON");
    }

    #[test]
    fn test_placeref_deserialize() {
        let _placeref : PlaceRef  = serde_json::from_str(EMPTY_JSON).expect("Could not parse Emoty JSON");
    }

    #[test]
    fn test_slafef_deserialize() {
        let _slaref : SLARef = serde_json::from_str(EMPTY_JSON).expect("Could not parse Emoty JSON")
    }

}

