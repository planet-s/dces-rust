use std::{cell::Cell, ops::Drop};

use crate::{
    entity::{Entity, EntityBuilder, EntityComponentManager, EntityContainer, VecEntityContainer},
    system::{EntitySystemBuilder, EntitySystemManager, System},
};

#[cfg(test)]
mod tests;

/// The `World` struct represents the main interface of the library. It used
/// as storage of entities, components and systems.
#[derive(Default)]
pub struct World<T>
where
    T: EntityContainer,
{
    entity_component_manager: EntityComponentManager,
    entity_system_manager: EntitySystemManager<T>,
    entity_counter: u32,
    entity_sytem_counter: u32,
    entity_container: T,
    first_run: bool,
}

impl<T> Drop for World<T>
where
    T: EntityContainer,
{
    fn drop(&mut self) {
        if let Some(cleanup_syste) = self.entity_system_manager.borrow_cleanup_system() {
            cleanup_syste
                .system
                .run(&self.entity_container, &mut self.entity_component_manager);
        }
    }
}

unsafe impl<T> Send for World<T> where T: EntityContainer {}

impl<T> World<T>
where
    T: EntityContainer,
{
    /// Creates a new world the a vector based entity container.
    pub fn new() -> World<VecEntityContainer> {
        World::from_container(VecEntityContainer::default())
    }

    /// Creates a new world from the given container.
    pub fn from_container(entity_container: T) -> Self {
        World {
            entity_component_manager: EntityComponentManager::new(),
            entity_system_manager: EntitySystemManager::new(),
            entity_counter: 0,
            entity_sytem_counter: 0,
            entity_container,
            first_run: true,
        }
    }

    /// Returns a mutable reference of the entity container.
    pub fn entity_container(&mut self) -> &mut T {
        &mut self.entity_container
    }

    /// Creates a new entity and returns a returns an `EntityBuilder`.
    pub fn create_entity(&mut self) -> EntityBuilder<'_, T> {
        let entity = self.entity_counter;
        self.entity_component_manager.register_entity(entity);
        self.entity_counter += 1;

        EntityBuilder {
            entity,
            entity_component_manager: &mut self.entity_component_manager,
            entity_container: &mut self.entity_container,
        }
    }

    /// Deletes the given `entity`.
    pub fn remove_entity(&mut self, entity: Entity) {
        self.entity_container.remove_entity(entity);
        self.entity_component_manager.remove_entity(entity);
    }

    /// Registers the init system.
    pub fn register_init_system(&mut self, init_system: impl System<T>) {
        self.entity_system_manager.register_init_system(init_system);
    }

    /// Registers the cleanup system.
    pub fn register_cleanup_system(&mut self, cleanup_system: impl System<T>) {
        self.entity_system_manager.register_cleanup_system(cleanup_system);
    }

    /// Creates a new entity system and returns a returns an `EntitySystemBuilder`.
    pub fn create_system(&mut self, system: impl System<T>) -> EntitySystemBuilder<'_, T> {
        let entity_system_id = self.entity_sytem_counter;
        self.entity_system_manager
            .register_system(system, entity_system_id);
        self.entity_sytem_counter += 1;

        EntitySystemBuilder {
            entity_system_manager: &mut self.entity_system_manager,
            entity_system_id,
            priority: Cell::new(0),
        }
    }

    /// Removes the given `entity`.
    pub fn remove_system(&mut self, system_id: u32) {
        self.entity_system_manager.remove_system(system_id);
    }

    /// Borrows mutable the entity component manager.
    pub fn entity_component_manager(&mut self) -> &mut EntityComponentManager {
        &mut self.entity_component_manager
    }

    /// Run all systems of the world.
    pub fn run(&mut self) {
        if self.first_run {
            if let Some(init_system) = self.entity_system_manager.borrow_init_system() {
                init_system
                    .system
                    .run(&self.entity_container, &mut self.entity_component_manager);
            }
            self.first_run = false;
        }

        let priorities = &self.entity_system_manager.priorities;
        for (_, prio) in priorities {
            for system in prio {
                self.entity_system_manager
                    .borrow_entity_system(*system)
                    .unwrap()
                    .system
                    .run(&self.entity_container, &mut self.entity_component_manager);
            }
        }
    }
}
