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

//! Product Recommendation Module

use serde::{Deserialize, Serialize};

use crate::common::tmf_error::TMFError;
use crate::{
    HasDescription, HasId, HasName, HasRelatedParty, HasValidity, TimePeriod, Uri,
};

use crate::tmf663::{cart_item::ItemRef, shopping_cart::ShoppingCartRef};

use crate::common::product::ProductRefOrValue;
use crate::common::related_party::RelatedParty;
use crate::common::related_place::RelatedPlaceRefOrValue;
#[cfg(feature = "build-V4")]
use crate::tmf620::product_offering::ProductOfferingRef;
#[cfg(feature = "build-V5")]
use crate::tmf620::product_offering_v5::ProductOfferingRef;
#[cfg(feature = "build-V4")]
use crate::tmf622::product_order_v4::ProductOrderRef;
#[cfg(feature = "build-V5")]
use crate::tmf622::product_order_v5::ProductOrderRef;

use super::MOD_PATH;

use tmflib_derive::{HasDescription, HasId, HasName, HasRelatedParty, HasValidity};

const CLASS_PATH: &str = "queryProductRecommendation";

/// Recommendation Item
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct RecommentationItem {
    /// Unique Identifier
    pub id: Option<String>,
    /// Recommendation Item Priority
    pub priority: u32,
    /// Product
    pub product: Option<Vec<ProductRefOrValue>>,
    /// Product Offering
    pub product_offering: Option<Vec<ProductOfferingRef>>,
}

/// Query Product Recommendations
#[derive(
    Clone,
    Default,
    Debug,
    HasId,
    HasName,
    HasDescription,
    HasValidity,
    HasRelatedParty,
    Deserialize,
    Serialize,
)]
pub struct QueryProductRecommendation {
    /// Unique Identifier
    pub id: Option<String>,
    /// HTTP Uri
    pub href: Option<Uri>,
    /// Name
    pub name: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Validity
    pub valid_for: Option<TimePeriod>,
    // Referenced objects
    /// Related Party
    pub related_party: Option<Vec<RelatedParty>>,
    /// Place
    pub place: Option<Vec<RelatedPlaceRefOrValue>>,
    /// Product
    pub product: Option<Vec<ProductRefOrValue>>,
    /// Product Order
    pub product_order: Option<Vec<ProductOrderRef>>,
    /// Recommandation Item
    pub recommendation_item: Option<Vec<RecommentationItem>>,
    /// Shopping Cart
    pub shopping_cart: Option<Vec<ShoppingCartRef>>,
    /// Shopping Cart Item
    pub shopping_cart_item: Option<Vec<ItemRef>>,
}

impl QueryProductRecommendation {
    /// Create a new Product Recommendation
    pub fn new(name: impl Into<String>) -> QueryProductRecommendation {
        QueryProductRecommendation {
            name: Some(name.into()),
            ..QueryProductRecommendation::create()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const REC_NAME: &str = "Recommendation Name";
    const REC_DESC: &str = "Recommendation Description";

    #[test]
    fn test_recommendation_create() {
        let recommendation = QueryProductRecommendation::new(REC_NAME).description(REC_DESC);

        assert_eq!(recommendation.get_name(), REC_NAME.to_string());
        assert_eq!(recommendation.get_description(), REC_DESC.to_string());
    }
}
