use std::cell::Cell;
use entity::{Entity, EntityBuilder, EntityComponentManager};
use system::{EntitySystemBuilder, EntitySystemManager, System};

#[cfg(test)]
mod tests;

/// The `World` struct represents the main interface of the library. It used
/// as storage of entities, components and systems.
#[derive(Default)]
pub struct World {
    entity_component_manager: EntityComponentManager,
    entity_system_manager: EntitySystemManager,
    entity_counter: u32,
    entity_sytem_counter: u32,
}

impl World {
    /// Creates a new world.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new entity and returns a returns an `EntityBuilder`.
    pub fn create_entity(&mut self) -> EntityBuilder {
        let entity = self.entity_counter;
        self.entity_component_manager.register_entity(entity);
        self.entity_counter += 1;

        EntityBuilder {
            entity,
            entity_component_manager: &mut self.entity_component_manager,
        }
    }

    /// Deletes the given `entity`.
    pub fn delete_entity(&mut self, entity: Entity) {
        self.entity_component_manager.remove_entity(entity);
        self.entity_system_manager.remove_entity(entity);
    }

    /// Creates a new entity system and returns a returns an `EntitySystemBuilder`.
    pub fn create_system<S: System>(&mut self, system: S) -> EntitySystemBuilder {
        let entity_system_id = self.entity_sytem_counter;
        self.entity_system_manager
            .register_system(system, entity_system_id);
        self.entity_sytem_counter += 1;

        EntitySystemBuilder {
            entity_system_manager: &mut self.entity_system_manager,
            entities: &self.entity_component_manager.entities,
            entity_system_id,
            priority: Cell::new(0),
        }
    }

    /// Removes the given `entity`.
    pub fn delete_system(&mut self, system_id: u32) {
        self.entity_system_manager.remove_system(system_id);
    }

    /// Apply all system filter and sort. This is only a temporary solution. The next version
    /// of the library will contains a implicit solution.
    pub fn apply_filter_and_sort(&mut self) {
        self.entity_system_manager.apply_filter_and_sort(&self.entity_component_manager.entities);
    }

    /// Run all systems of the world.
    pub fn run(&mut self) {
        let priorities = &self.entity_system_manager.priorities;
        for (_, prio) in priorities {
            for system in prio {
                let entities = &self
                    .entity_system_manager
                    .borrow_entity_system(*system)
                    .unwrap()
                    .entities;
                self.entity_system_manager
                    .borrow_entity_system(*system)
                    .unwrap()
                    .system
                    .run(entities, &mut self.entity_component_manager);
            }
        }
    }
}
