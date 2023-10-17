// Copyright 2023-2023 Ryan Ruckley.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

//! TMF620 Product Catalogue Module
//!
//! Structs associated with product catalogue management

use super::LIB_PATH;
use super::HasId;
use serde::{Deserialize, Serialize};

const MOD_PATH: &str = "tmf620";

pub mod bundled_product_offering;
pub mod catalog;
pub mod category;
pub mod party;
pub mod product_offering;
pub mod product_offering_price;
pub mod product_specification;

/// Agreement Reference
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AgreementRef {}

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

