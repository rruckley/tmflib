//! Resource Function Activation

use serde::{Deserialize, Serialize};

const MOD_PATH : &str = "resourceFunctionActivation/v4";

pub mod resource_function;
pub mod heal;
pub mod migrate;
pub mod monitor;
pub mod scale;

/// Task Status
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub enum TaskStateType {
    /// Task Accepted
    #[default]
    Accepted,
    /// Task failed with an error
    TerminatedWithError,
    /// Task is in progress
    InProgress,
    /// Task has completed
    Done,
}