use core::{
    any::Any,
    cell::Cell,
};

#[cfg(not(feature = "no_std"))]
use std::collections::{BTreeMap, HashMap};

#[cfg(feature = "no_std")]
use alloc::collections::{BTreeMap, HashMap};

use crate::{
    entity::{EntityComponentManager, EntityContainer},
    error::NotFound,
};

#[cfg(test)]
mod tests;

/// The run order of a system. The systems will be excuted by priority from small to great.
pub type Priority = i32;

/// This trait is used to interact with the components of entities. It could
/// read and write to the components.
pub trait System<T>: Any
where
    T: EntityContainer,
{
    fn run(&self, entities: &mut T, ecm: &mut EntityComponentManager);
}

/// Internal wrapper for a system. Contains also filter, priority, sort and entities.
pub struct EntitySystem<T> {
    /// The wrapped system.
    pub system: Box<dyn System<T>>,

    priority: Priority,
}

impl<T> EntitySystem<T> {
    /// Create a new entity system.
    pub fn new(system: Box<dyn System<T>>) -> Self {
        EntitySystem {
            system,
            priority: 0,
        }
    }
}

/// The entity system builder is used to create an entity system.
pub struct EntitySystemBuilder<'a, T>
where
    T: EntityContainer,
{
    /// Id of the entity system.
    pub entity_system_id: u32,

    /// Reference to the entity system manager, used to apply filter, sort and priority
    /// to the system.
    pub entity_system_manager: &'a mut EntitySystemManager<T>,

    // Priority of the entity system.
    pub priority: Cell<i32>,
}

impl<'a, T> EntitySystemBuilder<'a, T>
where
    T: EntityContainer,
{
    /// Add a `priority` to the system. Default priority is 0.
    pub fn with_priority(self, priority: Priority) -> Self {
        self.priority.set(priority);
        self
    }

    /// Finishing the creation of the system.
    pub fn build(self) -> u32 {
        self.entity_system_manager
            .register_priority(self.priority.get(), self.entity_system_id);
        self.entity_system_id
    }
}

/// The EntitySystemManager represents the main system storage.
#[derive(Default)]
pub struct EntitySystemManager<T>
where
    T: EntityContainer,
{
    // The entity systems.
    entity_systems: HashMap<u32, EntitySystem<T>>,

    // The init system.
    init_system: Option<EntitySystem<T>>,

    // The cleanup system.
    cleanup_system: Option<EntitySystem<T>>,

    /// Priorities of the systems.
    pub priorities: BTreeMap<i32, Vec<u32>>,
}

impl<T> EntitySystemManager<T>
where
    T: EntityContainer,
{
    /// Creates a new entity system manager with default values.
    pub fn new() -> Self {
        EntitySystemManager {
            entity_systems: HashMap::new(),
            init_system: None,
            cleanup_system: None,
            priorities: BTreeMap::new(),
        }
    }

    /// Registers the init system.
    pub fn register_init_system(&mut self, init_system: impl System<T>) {
        self.init_system = Some(EntitySystem::new(Box::new(init_system)));
    }

    /// Registers the cleanup system.
    pub fn register_cleanup_system(&mut self, cleanup_system: impl System<T>) {
        self.cleanup_system = Some(EntitySystem::new(Box::new(cleanup_system)));
    }

    /// Registers a new `system`.
    pub fn register_system(&mut self, system: impl System<T>, system_id: u32) {
        self.entity_systems
            .insert(system_id, EntitySystem::new(Box::new(system)));
    }

    /// Removes a system from the storage.
    pub fn remove_system(&mut self, system_id: u32) {
        self.entity_systems.remove(&system_id);
    }

    /// Register a `priority` for the system with the given `system_id`.
    pub fn register_priority(&mut self, priority: Priority, system_id: u32) {
        self.entity_systems.get_mut(&system_id).unwrap().priority = priority;

        // insert new priority and add system to it.
        if !self.priorities.contains_key(&priority) {
            self.priorities.insert(priority, vec![system_id]);
            return;
        }

        self.priorities.get_mut(&priority).unwrap().push(system_id);
    }

    /// Returns a reference of a entity system. If the entity system does not exists `NotFound` will be returned.
    pub fn borrow_entity_system(
        &self,
        entity_system_id: u32,
    ) -> Result<&EntitySystem<T>, NotFound> {
        self.entity_systems.get(&entity_system_id).map_or_else(
            || Err(NotFound::EntitySystem(entity_system_id)),
            |es| Ok(es),
        )
    }

    /// Returns a reference of the init entity system. If the init entity system does not exists `None` will be returned.
    pub fn borrow_init_system(
        &self,
    ) -> &Option<EntitySystem<T>> {
        &self.init_system
    }

    /// Returns a reference of the cleanup entity system. If the init entity system does not exists `None` will be returned.
    pub fn borrow_cleanup_system(
        &self,
    ) -> &Option<EntitySystem<T>> {
        &self.cleanup_system
    }
}
