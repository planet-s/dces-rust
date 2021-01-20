use core::any::{Any, TypeId};

#[cfg(feature = "no_std")]
use alloc::collections::{BTreeMap, FxHashMap};

use crate::entity::*;

pub use self::component_store::*;

mod component_store;

/// The entity builder is used to create an entity with components.
pub struct EntityBuilder<'a, E>
where
    E: EntityStore,
{
    /// The created entity.
    pub entity: Entity,

    /// Reference to the component store.
    pub component_store: &'a mut ComponentStore,

    /// Reference to the entity store.
    pub entity_store: &'a mut E,
}

impl<'a, E> EntityBuilder<'a, E>
where
    E: EntityStore,
{
    pub fn components(self, components: (BuildComponents, BuildSharedComponents)) -> Self {
        self.component_store.append(self.entity, components);
        self
    }
    /// Finishing the creation of the entity.
    pub fn build(self) -> Entity {
        self.entity_store.register_entity(self.entity);
        // self.component_store.register_entity(self.entity);
        self.entity
    }
}

/// This trait is used to internal handle all components types. This trait is implicitly implemented for all other types.
pub trait Component: Any {}
impl<E: Any> Component for E {}

/// This struct is used to store a component with its type id. Used for dynamic component adding.
pub struct ComponentBox {
    component: Box<dyn Any>,
    type_id: TypeId,
}

/// This struct is used to store a shared component with its type id. Used for dynamic component adding.
pub struct SharedComponentBox {
    source: Entity,
    type_id: TypeId,
}

impl SharedComponentBox {
    /// Creates the shared component box.
    pub fn new(type_id: TypeId, source: impl Into<Entity>) -> Self {
        SharedComponentBox {
            source: source.into(),
            type_id,
        }
    }

    /// Consumes the component box and returns the type id and the source.
    pub fn consume(self) -> (TypeId, Entity) {
        (self.type_id, self.source)
    }
}

impl ComponentBox {
    /// Creates the component box.
    pub fn new<C: Component>(component: C) -> Self {
        ComponentBox {
            component: Box::new(component),
            type_id: TypeId::of::<C>(),
        }
    }

    /// Consumes the component box and returns the type id and the component.
    pub fn consume(self) -> (TypeId, Box<dyn Any>) {
        (self.type_id, self.component)
    }
}

/// The EntityComponentManager represents the main entity and component storage.
#[derive(Default)]
pub struct EntityComponentManager<E>
where
    E: EntityStore,
{
    component_store: ComponentStore,

    entity_store: E,

    entity_counter: u32,
}

impl<E> EntityComponentManager<E>
where
    E: EntityStore,
{
    /// Create a new entity component manager.
    pub fn new(entity_store: E) -> Self {
        EntityComponentManager {
            entity_counter: 0,
            component_store: ComponentStore::default(),
            entity_store,
        }
    }

    /// Returns references to the component store and entity store.
    pub fn stores(&self) -> (&E, &ComponentStore) {
        (&self.entity_store, &self.component_store)
    }

    /// Returns mutable references to the component store and entity store.
    pub fn stores_mut(&mut self) -> (&mut E, &mut ComponentStore) {
        (&mut self.entity_store, &mut self.component_store)
    }

    /// Return a reference to the component container.
    pub fn component_store(&self) -> &ComponentStore {
        &self.component_store
    }

    /// Return a mutable reference to the component container.
    pub fn component_store_mut(&mut self) -> &mut ComponentStore {
        &mut self.component_store
    }

    /// Return a reference to the entity container.
    pub fn entity_store(&mut self) -> &mut E {
        &mut self.entity_store
    }

    /// Return a mutable reference to the entity container.
    pub fn entity_store_mut(&mut self) -> &mut E {
        &mut self.entity_store
    }

    /// Creates a new entity and returns a returns an `TypeEntityBuilder`.
    pub fn create_entity(&mut self) -> EntityBuilder<'_, E> {
        let entity: Entity = self.entity_counter.into();
        self.entity_counter += 1;

        EntityBuilder {
            entity,
            component_store: &mut self.component_store,
            entity_store: &mut self.entity_store,
        }
    }

    /// Register a new `entity`.
    pub fn register_entity(&mut self, entity: impl Into<Entity>) {
        let entity = entity.into();
        self.entity_store.register_entity(entity);
        // self.component_store.register_entity(entity);
    }

    /// Removes a `entity` from the manager.
    pub fn remove_entity(&mut self, entity: impl Into<Entity>) {
        let entity = entity.into();
        self.component_store.remove_entity(entity);
        self.entity_store.remove_entity(entity);
    }
}
