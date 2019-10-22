use core::{
    any::Any,
    cell::RefCell,
};

use std::collections::HashMap;

use crate::error::NotFound;
use super::{Entity, Component, EntityStore};

/// The type key based entity builder is used to create an entity with components.
pub struct StringEntityBuilder<'a, T> where T: EntityStore
{
    /// The created entity.
    pub entity: Entity,

    /// Reference to the component store.
    pub component_store: &'a mut StringComponentStore,

    /// Reference to the entity store.
    pub entity_store: &'a mut T,
}

impl<'a, T> StringEntityBuilder<'a, T> where
    T: EntityStore,
{
    /// Adds a component of type `C` to the entity.
    pub fn with<C: Component>(self, key: impl Into<String>, component: C) -> Self {
        self.component_store
            .register_component(self.entity, component, key);
        self
    }

    /// Adds an entity as `source` for a shared component of type `C`.
    pub fn with_shared<C: Component>(self, key: impl Into<String>, source: Entity) -> Self {
        self.component_store
            .register_shared_component::<C, String>(self.entity, source, key.into());
        self
    }

    /// Finishing the creation of the entity.
    pub fn build(self) -> Entity {
        self.entity_store.register_entity(self.entity);
        self.entity
    }
}

/// The `StringComponentStore` stores the components of entities and uses strings as component keys. It could be used to
/// borrow the components of the entities.
#[derive(Default, Debug)]
pub struct StringComponentStore {
    components: HashMap<Entity, HashMap<String, Box<dyn Any>>>,
    shared: HashMap<Entity, RefCell<HashMap<String, Entity>>>,
}

impl StringComponentStore {
    /// Registers an new entity on the store.
    pub fn register_entity(&mut self, entity: impl Into<Entity>) {
        self.components.insert(entity.into(), HashMap::new());
    }

    /// Removes and entity from the store.
    pub fn remove_entity(&mut self, entity: impl Into<Entity>) {
        self.components.remove(&entity.into());
    }

    /// Register a `component` for the given `entity`.
    pub fn register_component<C: Component>(&mut self, entity: Entity, component: C, key: impl Into<String>) {
        self.components
            .get_mut(&entity)
            .get_or_insert(&mut HashMap::new())
            .insert(key.into(), Box::new(component));
    }

    /// Registers a sharing of the given component between the given entities.
    pub fn register_shared_component<C: Component, K: Into<String>>(&mut self, target: Entity, source: Entity, key: K) {
        if !self.shared.contains_key(&target) {
            self.shared.insert(target, RefCell::new(HashMap::new()));
        }

        let key = key.into();

        // Removes unshared component of entity.
        if let Some(comp) = self.components.get_mut(&target) {
            comp.remove(&key);
        }

        self.shared[&target]
            .borrow_mut()
            .insert(key, source);
    }

    /// Returns the number of components in the store.
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Returns `true` if the store contains the specific entity.
    pub fn contains_entity(&self, entity: &Entity) -> bool {
        self.components.contains_key(entity)
    }

    /// Returns `true` if entity is the origin of the requested component `false`.
    pub fn is_origin<C: Component>(&self, entity: Entity, key: impl Into<String>) -> bool {
        if let Some(components) = self.components.get(&entity) {
            return components.contains_key(&key.into());
        }

        false
    }

    // Search the the target entity in the entity map.
    fn target_entity_from_shared(&self, entity: Entity, key: impl Into<String>) -> Result<Entity, NotFound> {
        let key = key.into();
        self.shared
            .get(&entity)
            .ok_or_else(|| NotFound::Entity(entity))
            .and_then(|en| {
                en.borrow()
                    .get(&key)
                    .map(|entity| *entity)
                    .ok_or_else(|| NotFound::ComponentKey(key))
            })
    }

    // Returns the target entity. First search in entities map. If not found search in shared entity map.
    fn target_entity(&self, entity: Entity, key: impl Into<String>) -> Result<Entity, NotFound> {
        let key = key.into();
        if !self.components.contains_key(&entity)
            || !self.components[&entity].contains_key(&key)
        {
            return self.target_entity_from_shared(entity, key);
        }

        Result::Ok(entity)
    }

    /// Returns a reference of a component of type `C` from the given `entity`. If the entity does
    /// not exists or it doesn't have a component of type `C` `NotFound` will be returned.
    pub fn borrow_component<C: Component>(&self, entity: Entity, key: impl Into<String>) -> Result<&C, NotFound> {
        let key = key.into();
        let target_entity = self.target_entity(entity, key.clone());

        match target_entity {
            Ok(entity) => self
                .components
                .get(&entity)
                .ok_or_else(|| NotFound::Entity(entity))
                .and_then(|en| {
                    en.get(&key)
                        .map(|component| {
                            component.downcast_ref().expect(
                                "StringComponentStore.borrow_component: internal downcast error",
                            )
                        })
                        .ok_or_else(|| NotFound::ComponentKey(key))
                }),
            Err(_) => Result::Err(NotFound::Entity(entity)),
        }
    }

    /// Returns a mutable reference of a component of type `C` from the given `entity`. If the entity does
    /// not exists or it doesn't have a component of type `C` `NotFound` will be returned.
    pub fn borrow_mut_component<C: Component>(
        &mut self,
        entity: Entity,
        key: impl Into<String>
    ) -> Result<&mut C, NotFound> {
        let key = key.into();
        let target_entity = self.target_entity(entity, key.clone());

        match target_entity {
            Ok(entity) => self
                .components
                .get_mut(&entity)
                .ok_or_else(|| NotFound::Entity(entity))
                .and_then(|en| {
                    en.get_mut(&key)
                        .map(|component| {
                            component.downcast_mut().expect(
                            "StringComponentStore.borrow_mut_component: internal downcast error",
                        )
                        })
                        .ok_or_else(|| NotFound::ComponentKey(key))
                }),
            Err(_) => Result::Err(NotFound::Entity(entity)),
        }
    }
}