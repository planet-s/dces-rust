use core::{any::Any, cell::Cell};

#[cfg(not(feature = "no_std"))]
use std::collections::{BTreeMap, HashMap};

#[cfg(feature = "no_std")]
use alloc::collections::{BTreeMap, HashMap};

use crate::{component::*, entity::*, error::NotFound};


/// Default type if you don't want to use a context.
#[derive(Default)]
pub struct PhantomContext;

/// The run order of a system. The systems will be executed by priority from small to great.
pub type Priority = i32;

/// This trait is used to interact with the components of entities. It could
/// read and write to the components.
pub trait System<E, C, Ctx>: Any
where
    E: EntityStore,
    C: ComponentStore,
{
    /// Runs the system and give access to the entity component manager.
    fn run(&self, _ecm: &mut EntityComponentManager<E, C>) {}

    /// Runs the system and give access to the entity component manager and context.
    fn run_with_context(&self, _ecm: &mut EntityComponentManager<E, C>, _ctx: &mut Ctx) {}
}

/// Internal wrapper for a system. Contains also filter, priority, sort and entities.
pub struct EntitySystem<E, C, Ctx> {
    /// The wrapped system.
    pub system: Box<dyn System<E, C, Ctx>>,

    priority: Priority,
}

impl<E, C, Ctx> EntitySystem<E, C, Ctx> {
    /// Create a new entity system.
    pub fn new(system: Box<dyn System<E, C, Ctx>>) -> Self {
        EntitySystem {
            system,
            priority: 0,
        }
    }
}

/// The system store builder is used to create a system.
pub struct SystemStoreBuilder<'a, E, C, Ctx>
where
    E: EntityStore,
    C: ComponentStore,
{
    /// Id of the entity system.
    pub entity_system_id: u32,

    /// Reference to the system store, used to apply filter, sort and priority
    /// to the system.
    pub system_store: &'a mut SystemStore<E, C, Ctx>,

    // Priority of the entity system.
    pub priority: Cell<i32>,
}

impl<'a, E, C, Ctx> SystemStoreBuilder<'a, E, C, Ctx>
where
    E: EntityStore,
    C: ComponentStore,
{
    /// Add a `priority` to the system. Default priority is 0.
    pub fn with_priority(self, priority: Priority) -> Self {
        self.priority.set(priority);
        self
    }

    /// Finishing the creation of the system.
    pub fn build(self) -> u32 {
        self.system_store
            .register_priority(self.priority.get(), self.entity_system_id);
        self.entity_system_id
    }
}

/// The SystemStore represents the main system storage.
#[derive(Default)]
pub struct SystemStore<E, C, Ctx>
where
    E: EntityStore,
    C: ComponentStore,
{
    // The entity systems.
    entity_systems: HashMap<u32, EntitySystem<E, C, Ctx>>,

    // The init system.
    init_system: Option<EntitySystem<E, C, Ctx>>,

    // The cleanup system.
    cleanup_system: Option<EntitySystem<E, C, Ctx>>,

    /// Priorities of the systems.
    pub priorities: BTreeMap<i32, Vec<u32>>,
}

impl<E, C, Ctx> SystemStore<E, C, Ctx>
where
    E: EntityStore,
    C: ComponentStore,
{
    /// Creates a new system store with default values.
    pub fn new() -> Self {
        SystemStore {
            entity_systems: HashMap::new(),
            init_system: None,
            cleanup_system: None,
            priorities: BTreeMap::new(),
        }
    }

    /// Registers the init system.
    pub fn register_init_system(&mut self, init_system: impl System<E, C, Ctx>) {
        self.init_system = Some(EntitySystem::new(Box::new(init_system)));
    }

    /// Registers the cleanup system.
    pub fn register_cleanup_system(&mut self, cleanup_system: impl System<E, C, Ctx>) {
        self.cleanup_system = Some(EntitySystem::new(Box::new(cleanup_system)));
    }

    /// Registers a new `system`.
    pub fn register_system(&mut self, system: impl System<E, C, Ctx>, system_id: u32) {
        self.entity_systems
            .insert(system_id, EntitySystem::new(Box::new(system)));
    }

    /// Removes a system from the storage.
    pub fn remove_system(&mut self, system_id: u32) {
        {
            let system_to_remove = self.entity_systems.get(&system_id).unwrap();
            self.priorities.remove(&system_to_remove.priority);
        }
        self.entity_systems.remove(&system_id);
    }

    /// Register a `priority` for the system with the given `system_id`.
    pub fn register_priority(&mut self, priority: Priority, system_id: u32) {
        self.entity_systems.get_mut(&system_id).unwrap().priority = priority;
        self.priorities
            .entry(priority)
            .or_insert_with(|| vec![])
            .push(system_id);
    }

    /// Returns a reference of a entity system. If the entity system does not exists `NotFound` will be returned.
    pub fn borrow_entity_system(
        &self,
        entity_system_id: u32,
    ) -> Result<&EntitySystem<E, C, Ctx>, NotFound> {
        self.entity_systems
            .get(&entity_system_id)
            .map_or_else(|| Err(NotFound::EntitySystem(entity_system_id)), Ok)
    }

    /// Returns a reference of the init entity system. If the init entity system does not exists `None` will be returned.
    pub fn borrow_init_system(&self) -> &Option<EntitySystem<E, C, Ctx>> {
        &self.init_system
    }

    /// Returns a reference of the cleanup entity system. If the init entity system does not exists `None` will be returned.
    pub fn borrow_cleanup_system(&self) -> &Option<EntitySystem<E, C, Ctx>> {
        &self.cleanup_system
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::TypeComponentStore;
    use crate::entity::VecEntityStore;

    struct TestSystem;

    impl System<VecEntityStore, TypeComponentStore, PhantomContext> for TestSystem {}

    #[test]
    fn test_register_system() {
        let mut esm = SystemStore::new();
        esm.register_system(TestSystem, 0);

        assert!(esm.entity_systems.contains_key(&0));
    }

    #[test]
    fn test_register_init_system() {
        let mut esm = SystemStore::new();

        assert!(esm.init_system.is_none());
        esm.register_init_system(TestSystem);

        assert!(esm.init_system.is_some());
    }

    #[test]
    fn test_register_cleanup_system() {
        let mut esm = SystemStore::new();

        assert!(esm.cleanup_system.is_none());
        esm.register_cleanup_system(TestSystem);

        assert!(esm.cleanup_system.is_some());
    }

    #[test]
    fn test_remove_system() {
        let mut esm = SystemStore::new();
        esm.register_system(TestSystem, 0);
        esm.remove_system(0);

        assert!(!esm.entity_systems.contains_key(&0));
        assert!(!esm.priorities.contains_key(&0));
    }

    #[test]
    fn test_register_priority() {
        let mut esm = SystemStore::new();
        esm.register_system(TestSystem, 0);
        esm.register_priority(5, 0);

        assert_eq!(esm.entity_systems.get(&0).unwrap().priority, 5);
        assert!(esm.priorities.contains_key(&5));
    }

    #[test]
    fn test_borrow_init_entity_system() {
        let mut esm = SystemStore::new();
        esm.register_init_system(TestSystem);

        assert!(esm.borrow_init_system().is_some());
    }

    #[test]
    fn test_borrow_cleanup_entity_system() {
        let mut esm = SystemStore::new();
        esm.register_cleanup_system(TestSystem);

        assert!(esm.borrow_cleanup_system().is_some());
    }

    #[test]
    fn test_borrow_entity_system() {
        let mut esm = SystemStore::new();
        esm.register_system(TestSystem, 0);

        assert!(esm.borrow_entity_system(0).is_ok());
    }

    #[test]
    fn test_build() {
        let mut esm = SystemStore::new();
        esm.register_system(TestSystem, 0);

        {
            let esb = SystemStoreBuilder {
                entity_system_id: 0,
                system_store: &mut esm,
                priority: Cell::new(0),
            };

            assert_eq!(esb.build(), 0);
        }
    }
}
