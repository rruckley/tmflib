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
//! Performance Measurement object model for TMF628 Performance Management

use super::{MeasurementCollectionJobRef, PerformanceMeasurementRelationship, MOD_PATH};
use crate::{common::entity::Entity, HasDescription, HasEntity, TimePeriod};
use serde::{Deserialize, Serialize};
use tmflib_derive::HasDescription;

const CLASS_PATH: &str = "measurement";

/// Performance Measurement
#[derive(Debug, Clone, Serialize, HasDescription, Deserialize, Default)]
pub struct PerformanceMeasurement {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///A free-text description of the performance measurement
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Reference to a MeasurementCollectionJob
    #[serde(rename = "measurementCollectionJob")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurement_collection_job: Option<MeasurementCollectionJobRef>,
    ///related Performance measurements array
    #[serde(rename = "relatedMeasurement")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_measurement: Vec<PerformanceMeasurementRelationship>,
    ///The optional tag object attached to this entire measurement
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<serde_json::Value>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}

impl HasEntity for PerformanceMeasurement {
    fn generate_id(&mut self) {
        let id = PerformanceMeasurement::get_uuid();
        self.id = id.into();
        self.generate_href();
    }
    fn generate_href(&mut self) {
        let href = format!(
            "{}/{}",
            PerformanceMeasurement::get_class_href(),
            self.get_id()
        );
        self.href = href.into();
    }

    fn get_id(&self) -> String {
        match self.id.as_ref() {
            Some(id) => id.clone(),
            None => String::default(),
        }
    }

    fn get_href(&self) -> String {
        match self.href.as_ref() {
            Some(href) => href.clone(),
            None => String::default(),
        }
    }

    fn get_class() -> String {
        CLASS_PATH.to_string()
    }

    fn get_class_href() -> String {
        format!(
            "/{}/{}/{}",
            crate::get_lib_path(),
            MOD_PATH,
            PerformanceMeasurement::get_class()
        )
    }

    fn get_mod_path() -> String {
        format!("/{}/{}", crate::get_lib_path(), MOD_PATH)
    }

    fn set_id(&mut self, id: impl Into<String>) {
        self.id = Some(id.into());
        self.generate_href();
    }

    fn id(mut self, id: impl Into<String>) -> Self {
        self.set_id(id);
        self
    }
}
impl std::fmt::Display for PerformanceMeasurement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PerformanceMeasurement {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for PerformanceMeasurement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
