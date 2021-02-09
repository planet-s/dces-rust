use core::cell::Cell;
use core::ops::Drop;

use crate::{
    component::*,
    entity::*,
    resources::Resources,
    system::{System, SystemStore, SystemStoreBuilder},
};

/// The `World` struct represents the main interface of the library. It used
/// as storage of entities, components and systems.
pub struct World<E>
where
    E: EntityStore,
{
    entity_component_manager: EntityComponentManager<E>,
    resources: Resources,
    system_store: SystemStore<E>,
    system_counter: u32,
    first_run: bool,
}

impl<E> Drop for World<E>
where
    E: EntityStore,
{
    fn drop(&mut self) {
        if let Some(cleanup_system) = self.system_store.borrow_cleanup_system() {
            cleanup_system
                .system
                .run(&mut self.entity_component_manager, &mut self.resources);
        }
    }
}

unsafe impl<E> Send for World<E> where E: EntityStore {}

impl<E> World<E>
where
    E: EntityStore,
{
    /// Creates a new world from the given entity store.
    pub fn from_entity_store(entity_store: E) -> Self {
        World {
            entity_component_manager: EntityComponentManager::new(entity_store),
            resources: Resources::default(),
            system_counter: 0,
            system_store: SystemStore::new(),
            first_run: true,
        }
    }

    /// Returns a reference to the resources store.
    pub fn resources(&self) -> &Resources {
        &self.resources
    }

    /// Returns a mutable reference to the resources store.
    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }

    /// Inserts a new resource.
    pub fn insert_resource<C: Component>(&mut self, resource: C) {
        self.resources.insert(resource);
    }

    /// Gets an element from the resources.
    pub fn resource<C: Component>(&self) -> &C {
        self.resources.get::<C>()
    }

    /// Gets a mutable reference of the requested element.
    pub fn resource_mut<C: Component>(&mut self) -> &mut C {
        self.resources.get_mut::<C>()
    }

    /// Try to get an element from the resources.
    pub fn try_resource<C: Component>(&self) -> Option<&C> {
        self.resources.try_get::<C>()
    }

    /// Try to get an element from the resources.
    pub fn try_resource_mut<C: Component>(&mut self) -> Option<&mut C> {
        self.resources.try_get_mut::<C>()
    }

    /// Returns `true` if the resources contains a resource of the given type overwise `false` .
    pub fn contains_resource<C: Component>(&self) -> bool {
        self.resources.contains::<C>()
    }

    /// Creates a new entity and returns a returns an `TypeEntityBuilder`.
    pub fn create_entity(&mut self) -> EntityBuilder<'_, E> {
        self.entity_component_manager.create_entity()
    }

    /// Deletes the given `entity`.
    pub fn remove_entity(&mut self, entity: impl Into<Entity>) {
        self.entity_component_manager.remove_entity(entity);
    }

    /// Registers the init system.
    pub fn register_init_system(&mut self, init_system: impl System<E>) {
        self.system_store.register_init_system(init_system);
    }

    /// Registers the cleanup system.
    pub fn register_cleanup_system(&mut self, cleanup_system: impl System<E>) {
        self.system_store.register_cleanup_system(cleanup_system);
    }

    /// Creates a new entity system and returns a returns an `SystemStoreBuilder`.
    pub fn create_system(&mut self, system: impl System<E>) -> SystemStoreBuilder<'_, E> {
        let entity_system_id = self.system_counter;
        self.system_store.register_system(system, entity_system_id);
        self.system_counter += 1;

        SystemStoreBuilder {
            system_store: &mut self.system_store,
            entity_system_id,
            priority: Cell::new(0),
        }
    }

    /// Removes the given `entity`.
    pub fn remove_system(&mut self, system_id: u32) {
        self.system_store.remove_system(system_id);
    }

    /// Borrows mutable the entity component manager.
    pub fn entity_component_manager(&mut self) -> &mut EntityComponentManager<E> {
        &mut self.entity_component_manager
    }

    /// Print infos about the given entity.
    pub fn print_entity(&self, entity: impl Into<Entity>) {
        self.entity_component_manager
            .component_store()
            .print_entity(entity);
    }

    /// Run all systems of the world.
    pub fn run(&mut self) {
        if self.first_run {
            if let Some(init_system) = self.system_store.borrow_init_system() {
                init_system
                    .system
                    .run(&mut self.entity_component_manager, &mut self.resources);
            }
            self.first_run = false;
        }

        let priorities = &self.system_store.priorities;
        for priority in priorities.values() {
            for system in priority {
                self.system_store
                    .borrow_entity_system(*system)
                    .unwrap()
                    .system
                    .run(&mut self.entity_component_manager, &mut self.resources);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{Entity, VecEntityStore};

    #[derive(Default)]
    struct TestSystem;

    impl System<VecEntityStore> for TestSystem {
        fn run(&self, _ecm: &mut EntityComponentManager<VecEntityStore>, _res: &mut Resources) {}
    }

    #[test]
    fn create_entity() {
        let mut world: World<VecEntityStore> = World::from_entity_store(VecEntityStore::default());
        assert_eq!(Entity(0), world.create_entity().build());
        assert_eq!(Entity(1), world.create_entity().build());
    }

    #[test]
    fn create_system() {
        let mut world = World::from_entity_store(VecEntityStore::default());
        assert_eq!(0, world.create_system(TestSystem).build());
        assert_eq!(1, world.create_system(TestSystem).build());
    }
}
