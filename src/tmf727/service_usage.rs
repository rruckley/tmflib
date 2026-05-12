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

//! TMF727 - Service Usage Module

use serde::{Serialize, Deserialize};
use super::{
    Characteristic, ExternalIdentifier, RelatedPartyRefOrPartyRoleRef,
    ResourceUsageRef, ServiceRef, ServiceUsageRef, ServiceUsageSpecificationRef,
};
use crate::common::entity::Entity;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceUsage {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Relationship to Service Usage in bundle
    #[serde(rename = "bundledServiceUsage")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bundled_service_usage: Vec<ServiceUsageRef>,
    ///Description of service usage
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///A list of external identifiers assoicated with this service
    #[serde(rename = "externalIdentifier")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_identifier: Vec<ExternalIdentifier>,
    ///isBundle determines whether usage represents a single usage (false), or a bundle of usages specifications (true).
    #[serde(rename = "isBundle")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_bundle: Option<bool>,
    ///A list of related party references (RelatedParty [*]). A related party defines party or party role linked to a specific entity
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_party: Vec<RelatedPartyRefOrPartyRoleRef>,
    ///Relationship to resource usage
    #[serde(rename = "resourceUsage")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_usage: Vec<ResourceUsageRef>,
    ///Service reference, for when Service is used by other entities.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceRef>,
    ///A list of characteristics that characterize this Service Usage (usageCharacteristic [*])
    #[serde(rename = "usageCharacteristic")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub usage_characteristic: Vec<Characteristic>,
    ///Date of usage
    #[serde(rename = "usageDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_date: Option<crate::DateTime>,
    ///UsageSpecification reference. UsageSpecification is a detailed description of a service usage event that are of interest to the business. It is comprised of characteristics, which define all attributes known for a particular type of usage.
    #[serde(rename = "usageSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_specification: Option<ServiceUsageSpecificationRef>,
    ///Type of usage
    #[serde(rename = "usageType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
}
impl std::fmt::Display for ServiceUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ServiceUsage {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for ServiceUsage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
