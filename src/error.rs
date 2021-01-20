use core::any::TypeId;

use crate::entity::Entity;

/// Not found error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotFound {
    /// Entity could not be found
    Entity(Entity),
    /// Component could not be found
    Component(TypeId),
    /// EntitySystem could not be found
    EntitySystem(u32),
    /// Component key could not be found
    ComponentKey(String),
    /// Unknown error
    Unknown(String),
    /// Key could not be found
    Key((Entity, String)),
}

impl Default for NotFound {
    fn default() -> Self {
        NotFound::Unknown(String::default())
    }
}
