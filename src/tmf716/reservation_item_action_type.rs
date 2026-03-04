use serde::{Deserialize, Serialize};
///action to be performed on the entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReservationItemActionType {
    ///Add a new entity
    #[serde(rename = "add")]
    Add,
    ///Modify an existing entity. The entity is identified by the id attribute.
    #[serde(rename = "modify")]
    Modify,
    ///Delete an existing entity. The entity is identified by the id attribute.
    #[serde(rename = "delete")]
    Delete,
    ///No change to the entity. The entity is identified by the id attribute.
    #[serde(rename = "noChange")]
    NoChange,
}
