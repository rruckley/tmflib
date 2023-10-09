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

//! TMF Library

#[warn(missing_docs)]

use chrono::naive::NaiveDateTime;
use chrono::Utc;
use uuid::Uuid;

/// Primary path for the whole library
pub const LIB_PATH: &str = "tmflib";

/// Trait indicating a TMF struct has and id and corresponding href field
pub trait HasId {
    fn get_uuid(&self) -> String {
        // Using simple format as SurrealDB doesn't like dashes in standard format.
        Uuid::new_v4().as_simple().to_string()
    }
    fn generate_id(&mut self);
    fn generate_href(&mut self);
    fn get_id(&mut self) -> String;
    fn get_href(&mut self) -> String;
}

/// Trait to create TMF structs that have the HasId trait
pub trait CreateTMF<T : Default + HasId> {
    fn create() -> T {
        // Create default instance
        let mut item = T::default();
        // Generate unique id
        item.generate_id();
        item
    }
}

/// Trait indicating a TMF sturct has a last_update or similar timestamp field.
pub trait HasLastUpdate {
    fn get_timestamp() -> String {
        let now = Utc::now();
        let time = NaiveDateTime::from_timestamp_opt(now.timestamp(), 0).unwrap();
        time.to_string()
    }
    fn set_last_update(&mut self, time : String);
}

/// Trait to create a TMF struct including a timestamp field
pub trait CreateTMFWithTime<T : Default + HasId + HasLastUpdate> {
    fn create_with_time() -> T {
        // Create default instance
        let mut item = T::default();
        item.generate_id();
        item.set_last_update(T::get_timestamp());
        item
    }
}

pub mod common;
pub mod tmf620;
pub mod tmf622;
pub mod tmf629;
pub mod tmf637;
pub mod tmf638;
pub mod tmf641;
pub mod tmf648;
pub mod tmf700;
