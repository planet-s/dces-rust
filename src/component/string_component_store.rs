use core::any::Any;

use std::collections::HashMap;

use super::{Component, ComponentStore, Entity};
use crate::error::NotFound;

/// The `StringComponentBuilder` is used to build a set of string key based components.
#[derive(Default)]
pub struct StringComponentBuilder {
    components: HashMap<String, Box<dyn Any>>,
    shared: HashMap<String, Entity>,
}

impl StringComponentBuilder {
    /// Creates an new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Adds a component of type `C` to the entity.
    pub fn with<C: Component>(mut self, key: impl Into<String>, component: C) -> Self {
        self.components.insert(key.into(), Box::new(component));
        self
    }

    /// Adds an entity as `source` for a shared component of type `C`.
    pub fn with_shared<C: Component>(mut self, key: impl Into<String>, source: Entity) -> Self {
        self.shared.insert(key.into(), source);
        self
    }

    /// Finishing the creation of the entity.
    pub fn build(self) -> (HashMap<String, Box<dyn Any>>, HashMap<String, Entity>) {
        (self.components, self.shared)
    }
}

/// The `StringComponentStore` stores the components of entities and uses strings as component keys. It could be used to
/// borrow the components of the entities.
#[derive(Default, Debug)]
pub struct StringComponentStore {
    components: HashMap<Entity, HashMap<String, Box<dyn Any>>>,
    shared: HashMap<Entity, HashMap<String, Entity>>,
}

impl ComponentStore for StringComponentStore {
    type Components = (HashMap<String, Box<dyn Any>>, HashMap<String, Entity>);

    fn append(&mut self, entity: Entity, components: Self::Components) {
        self.register_entity(entity);
        for (key, value) in components.0 {
            self.components.get_mut(&entity).unwrap().insert(key, value);
        }
        for (key, value) in components.1 {
            self.shared.get_mut(&entity).unwrap().insert(key, value);
        }
    }

    fn register_entity(&mut self, entity: impl Into<Entity>) {
        self.components.insert(entity.into(), HashMap::new());
    }

    fn remove_entity(&mut self, entity: impl Into<Entity>) {
        self.components.remove(&entity.into());
    }
}

impl StringComponentStore {
    /// Register a `component` for the given `entity`.
    pub fn register_component<C: Component>(
        &mut self,
        entity: Entity,
        component: C,
        key: impl Into<String>,
    ) {
        self.components
            .get_mut(&entity)
            .get_or_insert(&mut HashMap::new())
            .insert(key.into(), Box::new(component));
    }

    /// Registers a sharing of the given component between the given entities.
    pub fn register_shared_component<C: Component, K: Into<String>>(
        &mut self,
        target: Entity,
        source: Entity,
        key: K,
    ) {
        if !self.shared.contains_key(&target) {
            self.shared.insert(target, HashMap::new());
        }

        let key = key.into();

        // Removes unshared component of entity.
        if let Some(comp) = self.components.get_mut(&target) {
            comp.remove(&key);
        }

        self.shared.get_mut(&target).unwrap().insert(key, source);
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
    fn target_entity_from_shared(
        &self,
        entity: Entity,
        key: impl Into<String>,
    ) -> Result<Entity, NotFound> {
        let key = key.into();
        self.shared
            .get(&entity)
            .ok_or_else(|| NotFound::Entity(entity))
            .and_then(|en| {
                en.get(&key)
                    .map(|entity| *entity)
                    .ok_or_else(|| NotFound::ComponentKey(key))
            })
    }

    // Returns the target entity. First search in entities map. If not found search in shared entity map.
    fn target_entity(&self, entity: Entity, key: impl Into<String>) -> Result<Entity, NotFound> {
        let key = key.into();
        if !self.components.contains_key(&entity) || !self.components[&entity].contains_key(&key) {
            return self.target_entity_from_shared(entity, key);
        }

        Result::Ok(entity)
    }

    /// Returns a reference of a component of type `C` from the given `entity`. If the entity does
    /// not exists or it doesn't have a component of type `C` `NotFound` will be returned.
    pub fn borrow_component<C: Component>(
        &self,
        entity: Entity,
        key: impl Into<String>,
    ) -> Result<&C, NotFound> {
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
        key: impl Into<String>,
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
