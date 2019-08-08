use core::{
    any::{Any, TypeId},
    cell::RefCell,
};

// #![no_std]
use std::collections::HashMap;

#[cfg(feature = "no_std")]
use alloc::collections::{BTreeMap, HashMap};

use crate::error::NotFound;

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

/// The entity builder is used to create an entity with components.
pub struct EntityBuilder<'a, T>
where
    T: EntityStore,
{
    /// The created entity.
    pub entity: Entity,

    /// Reference to the entity component manager, used to add components
    /// to the entity.
    pub entity_component_manager: &'a mut EntityComponentManager<T>,
}

impl<'a, T> EntityBuilder<'a, T>
where
    T: EntityStore,
{
    /// Adds a component of type `C` to the entity.
    pub fn with<C: Component>(self, component: C) -> Self {
        self.entity_component_manager
            .register_component(self.entity, component);
        self
    }

    /// Adds an entity as `source` for a shared component of type `C`.
    pub fn with_shared<C: Component>(self, source: Entity) -> Self {
        self.entity_component_manager
            .register_shared_component::<C>(self.entity, source);
        self
    }

    /// Adds an entity as `source` for a shared component box.
    pub fn with_shared_box(self, source: SharedComponentBox) -> Self {
        self.entity_component_manager
            .register_shared_component_box(self.entity, source);
        self
    }

    /// Adds a component box to the entity.
    pub fn with_box(self, component_box: ComponentBox) -> Self {
        self.entity_component_manager
            .register_component_box(self.entity, component_box);
        self
    }

    /// Finishing the creation of the entity.
    pub fn build(self) -> Entity {
        self.entity_component_manager
            .entity_store
            .register_entity(self.entity);
        self.entity
    }
}

/// The EntityComponentManager represents the main entity and component storage.
#[derive(Default)]
pub struct EntityComponentManager<T>
where
    T: EntityStore,
{
    component_store: ComponentStore,

    entity_store: T,
}

impl<T> EntityComponentManager<T>
where
    T: EntityStore,
{
    /// Create a new entity component manager.
    pub fn new(entity_store: T) -> Self {
        EntityComponentManager {
            component_store: ComponentStore::default(),
            entity_store,
        }
    }

    /// Returns references to the component store and entity store.
    pub fn stores(&self) -> (&T, &ComponentStore) {
        (&self.entity_store, &self.component_store)
    }

    /// Returns mutable references to the component store and entity store.
    pub fn stores_mut(&mut self) -> (&mut T, &mut ComponentStore) {
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
    pub fn entity_store(&mut self) -> &mut T {
        &mut self.entity_store
    }

    /// Return a mutable reference to the entity container.
    pub fn entity_store_mut(&mut self) -> &mut T {
        &mut self.entity_store
    }

    /// Creates a new entity and returns a returns an `EntityBuilder`.
    pub fn create_entity(&mut self) -> EntityBuilder<'_, T> {
        let entity: Entity = self.component_store.create_entity();

        EntityBuilder {
            entity,
            entity_component_manager: self,
        }
    }

    /// Register a new `entity`.
    pub fn register_entity(&mut self, entity: impl Into<Entity>) {
        self.component_store
            .components
            .insert(entity.into(), HashMap::new());
    }

    /// Removes a `entity` from the manager.
    pub fn remove_entity(&mut self, entity: impl Into<Entity>) {
        let entity = entity.into();
        self.component_store.components.remove(&entity);
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

/// The `ComponentStore` stores the components of all entities. It could be used to
/// borrow the components of the entities.
#[derive(Default, Debug)]
pub struct ComponentStore {
    components: HashMap<Entity, HashMap<TypeId, Box<dyn Any>>>,
    shared: HashMap<Entity, RefCell<HashMap<TypeId, Entity>>>,
    entity_counter: u32,
}

impl ComponentStore {
    pub fn create_entity(&mut self) -> Entity {
        let entity: Entity = self.entity_counter.into();
        self.components.insert(entity, HashMap::new());
        self.entity_counter += 1;
        entity
    }

    /// Register a `component` for the given `entity`.
    pub fn register_component<C: Component>(&mut self, entity: Entity, component: C) {
        self.components
            .get_mut(&entity)
            .get_or_insert(&mut HashMap::new())
            .insert(TypeId::of::<C>(), Box::new(component));
    }

    /// Registers a sharing of the given component between the given entities.
    pub fn register_shared_component<C: Component>(&mut self, target: Entity, source: Entity) {
        if !self.shared.contains_key(&target) {
            self.shared.insert(target, RefCell::new(HashMap::new()));
        }

        self.shared[&target]
            .borrow_mut()
            .insert(TypeId::of::<C>(), source);
    }

    /// Registers a sharing of the given component between the given entities.
    pub fn register_shared_component_box(
        &mut self,
        target: impl Into<Entity>,
        source: SharedComponentBox,
    ) {
        let target = target.into();
        if !self.shared.contains_key(&target) {
            self.shared.insert(target, RefCell::new(HashMap::new()));
        }

        self.shared[&target]
            .borrow_mut()
            .insert(source.type_id, source.source);
    }

    /// Register a `component_box` for the given `entity`.
    pub fn register_component_box(
        &mut self,
        entity: impl Into<Entity>,
        component_box: ComponentBox,
    ) {
        let entity = entity.into();
        let (type_id, component) = component_box.consume();

        self.components
            .get_mut(&entity)
            .get_or_insert(&mut HashMap::new())
            .insert(type_id, component);
    }

    /// Returns the number of components in the store.
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Returns `true` if the store contains the specific entity.
    pub fn contains_entity(&self, entity: &Entity) -> bool {
        self.components.contains_key(entity)
    }

    // Search the the target entity in the entity map.
    fn target_entity_from_shared<C: Component>(&self, entity: Entity) -> Result<Entity, NotFound> {
        self.shared
            .get(&entity)
            .ok_or_else(|| NotFound::Entity(entity))
            .and_then(|en| {
                en.borrow()
                    .get(&TypeId::of::<C>())
                    .map(|entity| *entity)
                    .ok_or_else(|| NotFound::Component(TypeId::of::<C>()))
            })
    }

    // Returns the target entity. First search in entities map. If not found search in shared entity map.
    fn target_entity<C: Component>(&self, entity: Entity) -> Result<Entity, NotFound> {
        if !self.components.contains_key(&entity)
            || !self.components[&entity].contains_key(&TypeId::of::<C>())
        {
            return self.target_entity_from_shared::<C>(entity);
        }

        Result::Ok(entity)
    }

    /// Returns a reference of a component of type `C` from the given `entity`. If the entity does
    /// not exists or it doesn't have a component of type `C` `NotFound` will be returned.
    pub fn borrow_component<C: Component + Default>(&self, entity: Entity) -> Result<&C, NotFound> {
        let target_entity = self.target_entity::<C>(entity);

        match target_entity {
            Ok(entity) => self
                .components
                .get(&entity)
                .ok_or_else(|| NotFound::Entity(entity))
                .and_then(|en| {
                    en.get(&TypeId::of::<C>())
                        .map(|component| {
                            component.downcast_ref().expect(
                                "EntityComponentManager.borrow_component: internal downcast error",
                            )
                        })
                        .ok_or_else(|| NotFound::Component(TypeId::of::<C>()))
                }),
            Err(_) => Result::Err(NotFound::Entity(entity)),
        }
    }

    /// Returns a mutable reference of a component of type `C` from the given `entity`. If the entity does
    /// not exists or it doesn't have a component of type `C` `NotFound` will be returned.
    pub fn borrow_mut_component<C: Component + Default>(
        &mut self,
        entity: Entity,
    ) -> Result<&mut C, NotFound> {
        let target_entity = self.target_entity::<C>(entity);

        match target_entity {
            Ok(entity) => self
                .components
                .get_mut(&entity)
                .ok_or_else(|| NotFound::Entity(entity))
                .and_then(|en| {
                    en.get_mut(&TypeId::of::<C>())
                        .map(|component| {
                            component.downcast_mut().expect(
                            "EntityComponentManager.borrow_mut_component: internal downcast error",
                        )
                        })
                        .ok_or_else(|| NotFound::Component(TypeId::of::<C>()))
                }),
            Err(_) => Result::Err(NotFound::Entity(entity)),
        }
    }
}
