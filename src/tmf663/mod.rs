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

//! TMF663 Shopping Cart Management
//! # Description
//! Provides schema for management of [`shopping_cart::ShoppingCart`]s.
//! # Versions
//! - v4 supported

const MOD_PATH: &str = "shoppingCart/v4";

pub mod cart_item;
pub mod shopping_cart;
