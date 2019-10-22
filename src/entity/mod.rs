use core::{
    any::{Any, TypeId},
};

#[cfg(feature = "no_std")]
use alloc::collections::{BTreeMap, HashMap};

pub use self::string_component_store::*;
pub use self::type_component_store::*;

mod string_component_store;
mod type_component_store;

#[cfg(test)]
mod tests;

/// Represents an entity.
#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug, Ord, PartialOrd, Default)]
pub struct Entity(pub u32);

impl From<u32> for Entity {
    fn from(u: u32) -> Self {
        Entity(u)
    }
}

/// This trait is used to internal handle all components types. This trait is implicitly implemented for all other types.
pub trait Component: Any {}
impl<T: Any> Component for T {}

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
pub struct EntityComponentManager<T>
where
    T: EntityStore,
{
    component_store: TypeComponentStore,

    entity_store: T,

    entity_counter: u32,
}

impl<T> EntityComponentManager<T>
where
    T: EntityStore,
{
    /// Create a new entity component manager.
    pub fn new(entity_store: T) -> Self {
        EntityComponentManager {
            entity_counter: 0,
            component_store: TypeComponentStore::default(),
            entity_store,
        }
    }

    /// Returns references to the component store and entity store.
    pub fn stores(&self) -> (&T, &TypeComponentStore) {
        (&self.entity_store, &self.component_store)
    }

    /// Returns mutable references to the component store and entity store.
    pub fn stores_mut(&mut self) -> (&mut T, &mut TypeComponentStore) {
        (&mut self.entity_store, &mut self.component_store)
    }

    /// Return a reference to the component container.
    pub fn component_store(&self) -> &TypeComponentStore {
        &self.component_store
    }

    /// Return a mutable reference to the component container.
    pub fn component_store_mut(&mut self) -> &mut TypeComponentStore {
        &mut self.component_store
    }

    /// Return a reference to the entity container.
    pub fn entity_store(&mut self) -> &mut T {
        &mut self.entity_store
    }

    /// Return a mutable reference to the entity container.
    pub fn entity_store_mut(&mut self) -> &mut T {
        &mut self.entity_store
    }

    /// Creates a new entity and returns a returns an `TypeEntityBuilder`.
    pub fn create_entity(&mut self) -> TypeEntityBuilder<'_, T> {
        let entity: Entity = self.entity_counter.into();

        self.component_store
            .register_entity(entity);
        self.entity_counter += 1;

        TypeEntityBuilder {
            entity,
            component_store: &mut self.component_store,
            entity_store: &mut self.entity_store
        }
    }

    /// Register a new `entity`.
    pub fn register_entity(&mut self, entity: impl Into<Entity>) {
       self.component_store
            .register_entity(entity);
    }

    /// Removes a `entity` from the manager.
    pub fn remove_entity(&mut self, entity: impl Into<Entity>) {
        let entity = entity.into();
        self.component_store.remove_entity(entity);
        self.entity_store.remove_entity(entity);
    }

    /// Register a `component` for the given `entity`.
    pub fn register_component<C: Component>(&mut self, entity: Entity, component: C) {
        self.component_store.register_component(entity, component);
    }

    /// Registers a sharing of the given component between the given entities.
    pub fn register_shared_component<C: Component>(&mut self, target: Entity, source: Entity) {
        self.component_store
            .register_shared_component::<C>(target, source);
    }

    /// Registers a sharing of the given component between the given entities.
    pub fn register_shared_component_box(
        &mut self,
        target: impl Into<Entity>,
        source: SharedComponentBox,
    ) {
        self.component_store
            .register_shared_component_box(target, source);
    }

    /// Register a `component_box` for the given `entity`.
    pub fn register_component_box(
        &mut self,
        entity: impl Into<Entity>,
        component_box: ComponentBox,
    ) {
        self.component_store
            .register_component_box(entity, component_box);
    }
}

/// This trait is used to define a custom store for entities.
/// A entity container is used for entity iteration inside of the
/// system's run methods.
pub trait EntityStore {
    /// Registers the give 'entity'.
    fn register_entity(&mut self, entity: impl Into<Entity>);

    /// Removes the given 'entity'.
    fn remove_entity(&mut self, entity: impl Into<Entity>);
}

/// VecEntityStore is the default vector based implementation of an entity store.
#[derive(Default)]
pub struct VecEntityStore {
    pub inner: Vec<Entity>,
}

impl EntityStore for VecEntityStore {
    fn register_entity(&mut self, entity: impl Into<Entity>) {
        self.inner.push(entity.into());
    }

    fn remove_entity(&mut self, entity: impl Into<Entity>) {
        let entity = entity.into();
        self.inner
            .iter()
            .position(|&n| n == entity)
            .map(|e| self.inner.remove(e));
    }
}



