use core::cell::Cell;
use core::ops::Drop;

use crate::{
    component::*,
    entity::*,
    system::{System, SystemStore, SystemStoreBuilder},
};

/// The `World` struct represents the main interface of the library. It used
/// as storage of entities, components and systems.
pub struct World<E, C, Ctx>
where
    E: EntityStore,
    C: ComponentStore,
{
    entity_component_manager: EntityComponentManager<E, C>,
    system_store: SystemStore<E, C, Ctx>,
    system_counter: u32,
    first_run: bool,
}

impl<E, C, Ctx> Drop for World<E, C, Ctx>
where
    E: EntityStore,
    C: ComponentStore,
{
    fn drop(&mut self) {
        if let Some(cleanup_system) = self.system_store.borrow_cleanup_system() {
            cleanup_system
                .system
                .run(&mut self.entity_component_manager);
        }
    }
}

unsafe impl<E, C, Ctx> Send for World<E, C, Ctx>
where
    E: EntityStore,
    C: ComponentStore,
{
}

impl<E, C, Ctx> World<E, C, Ctx>
where
    E: EntityStore,
    C: ComponentStore,
{
    /// Creates a new world from the given container.
    // pub fn from_stores(entity_store: E, component_store: C) -> World<E, C, NullContext> {
    //    World::inner_from_stores::<NullContext>(entity_store, component_store)
    // }

    pub fn from_stores(entity_store: E, component_store: C) -> Self {
        World {
            entity_component_manager: EntityComponentManager::new(entity_store, component_store),
            system_counter: 0,
            system_store: SystemStore::new(),
            first_run: true,
        }
    }

    /// Creates a new entity and returns a returns an `TypeEntityBuilder`.
    pub fn create_entity(&mut self) -> EntityBuilder<'_, E, C> {
        self.entity_component_manager.create_entity()
    }

    /// Deletes the given `entity`.
    pub fn remove_entity(&mut self, entity: impl Into<Entity>) {
        self.entity_component_manager.remove_entity(entity);
    }

    /// Registers the init system.
    pub fn register_init_system(&mut self, init_system: impl System<E, C, Ctx>) {
        self.system_store.register_init_system(init_system);
    }

    /// Registers the cleanup system.
    pub fn register_cleanup_system(&mut self, cleanup_system: impl System<E, C, Ctx>) {
        self.system_store.register_cleanup_system(cleanup_system);
    }

    /// Creates a new entity system and returns a returns an `SystemStoreBuilder`.
    pub fn create_system(&mut self, system: impl System<E, C, Ctx>) -> SystemStoreBuilder<'_, E, C, Ctx> {
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
    pub fn entity_component_manager(&mut self) -> &mut EntityComponentManager<E, C> {
        &mut self.entity_component_manager
    }

    /// Print infos about the given entity.
    pub fn print_entity(&self, entity: impl Into<Entity>) {
        self.entity_component_manager.component_store().print_entity(entity);
    }

    /// Run all systems of the world.
    pub fn run(&mut self) {
        if self.first_run {
            if let Some(init_system) = self.system_store.borrow_init_system() {
                init_system.system.run(&mut self.entity_component_manager);
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
                    .run(&mut self.entity_component_manager);
            }
        }
    }

    /// Run all systems of the world and calls `run_with_context` of the systems with the given context.
    pub fn run_with_context(&mut self, ctx: &mut Ctx) {
        if self.first_run {
            if let Some(init_system) = self.system_store.borrow_init_system() {
                init_system.system.run_with_context(&mut self.entity_component_manager, ctx);
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
                    .run_with_context(&mut self.entity_component_manager, ctx);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::TypeComponentStore;
    use crate::entity::{Entity, VecEntityStore};
    use crate::system::NullContext;

    #[derive(Default)]
    struct TestSystem;

    impl System<VecEntityStore, TypeComponentStore, NullContext> for TestSystem {
        fn run(&self, _ecm: &mut EntityComponentManager<VecEntityStore, TypeComponentStore>) {}
    }

    #[test]
    fn create_entity() {
        let mut world: World<VecEntityStore, TypeComponentStore, NullContext> =
            World::from_stores(VecEntityStore::default(), TypeComponentStore::default());
        assert_eq!(Entity(0), world.create_entity().build());
        assert_eq!(Entity(1), world.create_entity().build());
    }

    #[test]
    fn create_system() {
        let mut world =
            World::from_stores(VecEntityStore::default(), TypeComponentStore::default());
        assert_eq!(0, world.create_system(TestSystem).build());
        assert_eq!(1, world.create_system(TestSystem).build());
    }
}
