use core::any::Any;

use std::collections::HashMap;

use super::{Component, ComponentBox, ComponentStore, Entity, SharedComponentBox};
use crate::error::NotFound;

/// The `StringComponentBuilder` is used to build a set of string key based components.
#[derive(Default)]
pub struct StringComponentBuilder {
    components: HashMap<String, Box<dyn Any>>,
    shared: HashMap<String, (Entity, String)>,
}

impl StringComponentBuilder {
    /// Creates an new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Adds a component of type `C` to the entity.
    pub fn with<C: Component>(mut self, key: &str, component: C) -> Self {
        self.components.insert(key.into(), Box::new(component));
        self
    }

    /// Adds an entity as `source` for a shared component of type `C`.
    pub fn with_shared<C: Component>(mut self, key: &str, source: Entity) -> Self {
        self.shared.insert(key.into(), (source, key.into()));
        self
    }

    /// Adds an entity as `source` for a shared component of type `C`.
    pub fn with_shared_source_key<C: Component>(
        mut self,
        key: &str,
        source_key: &str,
        source: Entity,
    ) -> Self {
        self.shared.insert(key.into(), (source, source_key.into()));
        self
    }

    /// Finishing the creation of the entity.
    pub fn build(
        self,
    ) -> (
        HashMap<String, Box<dyn Any>>,
        HashMap<String, (Entity, String)>,
    ) {
        (self.components, self.shared)
    }
}

/// The `StringComponentStore` stores the components of entities and uses strings as component keys. It could be used to
/// borrow the components of the entities.
#[derive(Default, Debug)]
pub struct StringComponentStore {
    components: HashMap<(Entity, String), Box<dyn Any>>,
    shared: HashMap<(Entity, String), (Entity, String)>,
}

impl ComponentStore for StringComponentStore {
    type Components = (
        HashMap<String, Box<dyn Any>>,
        HashMap<String, (Entity, String)>,
    );

    fn append(&mut self, entity: Entity, components: Self::Components) {
        for (key, value) in components.0 {
            self.components.insert((entity, key), value);
        }
        for (key, value) in components.1 {
            self.shared.insert((entity, key), (value.0, value.1));
        }
    }

    fn remove_entity(&mut self, entity: impl Into<Entity>) {
        let entity = entity.into();
        let keys: Vec<(Entity, String)> = self
            .components
            .iter()
            .filter(|&(k, _)| k.0 == entity.into())
            .map(|(k, _)| k.clone())
            .collect();

        for k in keys {
            self.components.remove(&k);
        }

        let keys: Vec<(Entity, String)> = self
            .shared
            .iter()
            .filter(|&(k, _)| k.0 == entity.into())
            .map(|(k, _)| k.clone())
            .collect();

        for k in keys {
            self.shared.remove(&k);
        }
    }
}

impl StringComponentStore {
    /// Register a `component` for the given `entity`.
    pub fn register_component<C: Component>(
        &mut self,
        key: impl Into<String>,
        entity: Entity,
        component: C,
    ) {
        self.components
            .insert((entity, key.into()), Box::new(component));
    }

    /// Registers a sharing of the given component between the given entities.
    pub fn register_shared_component<C: Component>(
        &mut self,
        key: &str,
        source_key: &str,
        target: Entity,
        source: Entity,
    ) {
        let target_key = (target, key.to_string());
        self.components.remove(&target_key);
        self.shared
            .insert(target_key, (source, source_key.to_string()));
    }

    /// Registers a sharing of the given component between the given entities.
    pub fn register_shared_component_box(
        &mut self,
        key: &str,
        source_key: &str,
        target: Entity,
        source: SharedComponentBox,
    ) {
        let target_key = (target, key.to_string());
        self.components.remove(&target_key);
        self.shared
            .insert(target_key, (source.source, source_key.to_string()));
    }

    /// Register a `component_box` for the given `entity`.
    pub fn register_component_box(
        &mut self,
        key: &str,
        entity: Entity,
        component_box: ComponentBox,
    ) {
        let (_, component) = component_box.consume();
        self.components.insert((entity, key.into()), component);
    }

    /// Returns the number of components in the store.
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Returns true if the components are empty.
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Returns `true` if the store contains the specific entity.
    pub fn contains_entity(&self, entity: Entity) -> bool {
        self.components.iter().any(|(k, _)| k.0 == entity)
    }

    /// Returns `true` if entity is the origin of the requested component `false`.
    pub fn is_origin<C: Component>(&self, key: &str, entity: Entity) -> bool {
        self.components.contains_key(&(entity, key.to_string()))
    }

    // Search the the source in the entity map.
    fn source_from_shared(
        &self,
        key: impl Into<String>,
        entity: Entity,
    ) -> Result<(Entity, String), NotFound> {
        self.shared
            .get(&(entity, key.into()))
            .ok_or_else(|| NotFound::Entity(entity))
            .map(|s| s.clone())
    }

    // Returns the source. First search in entities map. If not found search in shared entity map.
    fn source(&self, entity: Entity, key: impl Into<String>) -> Result<(Entity, String), NotFound> {
        let key = (entity, key.into());
        if !self.components.contains_key(&key) {
            return self.source_from_shared(key.1, key.0);
        }

        Result::Ok(key)
    }

    /// Returns a reference of a component of type `C` from the given `entity`. If the entity does
    /// not exists or it doesn't have a component of type `C` `NotFound` will be returned.
    pub fn get<C: Component>(&self, key: &str, entity: Entity) -> Result<&C, NotFound> {
        let source = self.source(entity, key);

        match source {
            Ok(source) => self
                .components
                .get(&(source.0, source.1))
                .ok_or_else(|| NotFound::Entity(entity))
                .map(|component| {
                    component
                        .downcast_ref()
                        .expect("StringComponentStore.get: internal downcast error")
                }),
            Err(_) => Result::Err(NotFound::Entity(entity)),
        }
    }

    /// Returns a mutable reference of a component of type `C` from the given `entity`. If the entity does
    /// not exists or it doesn't have a component of type `C` `NotFound` will be returned.
    pub fn get_mut<C: Component>(&mut self, key: &str, entity: Entity) -> Result<&mut C, NotFound> {
        let source = self.source(entity, key);

        match source {
            Ok(source) => self
                .components
                .get_mut(&(source.0, source.1))
                .ok_or_else(|| NotFound::Entity(entity))
                .map(|component| {
                    component
                        .downcast_mut()
                        .expect("StringComponentStore.get_mut: internal downcast error")
                }),
            Err(_) => Result::Err(NotFound::Entity(entity)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_with() {
        let builder = StringComponentBuilder::new();
        let component = String::from("Test");
        let (map, _) = builder.with("test", component).build();

        assert!(map.contains_key(&String::from("test")));
    }

    #[test]
    fn builder_with_shared() {
        let builder = StringComponentBuilder::default();
        let source = Entity::from(1);
        let (_, map) = builder.with_shared::<String>("test", source).build();

        assert!(map.contains_key(&String::from("test")));
        assert_eq!(
            *map.get(&String::from("test")).unwrap(),
            (source, String::from("test"))
        );
    }

    #[test]
    fn remove_entity() {
        let mut store = StringComponentStore::default();
        let entity = Entity::from(1);
        store.register_component("test", entity, String::from("Test"));
        store.remove_entity(entity);

        assert!(!store.contains_entity(entity));
    }

    #[test]
    fn register_component() {
        let mut store = StringComponentStore::default();
        let entity = Entity::from(1);
        let component = String::from("Test");

        store.register_component("test", entity, component);

        assert!(store.get::<String>("test", entity).is_ok());
    }

    #[test]
    fn len() {
        let mut store = StringComponentStore::default();
        let entity = Entity::from(1);

        store.register_component("string", entity, String::from("Test"));
        store.register_component("float", entity, 5 as f64);

        assert_eq!(store.len(), 2);
    }

    #[test]
    fn register_shared_component() {
        let mut store = StringComponentStore::default();
        let entity = Entity::from(1);
        let target = Entity::from(2);
        let target_next = Entity::from(3);
        let component = String::from("Test");

        store.register_component("test", entity, component);
        store.register_shared_component::<String>("test", "test", target, entity);
        store.register_shared_component::<String>("test_next", "test", target_next, entity);

        assert!(store.get::<String>("test", entity).is_ok());
        assert!(store.get::<String>("test", target).is_ok());
        assert!(store.get::<String>("test_next", target_next).is_ok());
        assert!(store.is_origin::<String>("test", entity));
        assert!(!store.is_origin::<String>("test", target));
        assert!(!store.is_origin::<String>("test", target_next));
    }
}
