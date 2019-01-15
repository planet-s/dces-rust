use std::any::TypeId;

use crate::entity::Entity;

/// Not found error.
#[derive(Debug, PartialEq, Eq)]
pub enum NotFound {
    /// Entity could not be found
    Entity(Entity),
    /// Component could not be found
    Component(TypeId),
    /// EntitySystem could not be found
    EntitySystem(u32),
    /// Unkown error
    Unkown(String)
}
