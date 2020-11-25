use core::any::Any;

use fxhash::FxHashMap;

use super::{Component, ComponentBox, ComponentStore, Entity, SharedComponentBox};
use crate::error::NotFound;

type BuildComponents = FxHashMap<String, Box<dyn Any>>;
type BuildSharedComponents = FxHashMap<String, (Entity, String)>;
type Components = FxHashMap<(Entity, String), Box<dyn Any>>;
type SharedComponents = FxHashMap<(Entity, String), (Entity, String)>;

/// The `StringComponentBuilder` is used to build a set of string key based components.
#[derive(Default)]
pub struct StringComponentBuilder {
    components: FxHashMap<String, Box<dyn Any>>,
    shared: FxHashMap<String, (Entity, String)>,
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
    pub fn build(self) -> (BuildComponents, BuildSharedComponents) {
        (self.components, self.shared)
    }
}

/// The `StringComponentStore` stores the components of entities and uses strings as component keys. It could be used to
/// borrow the components of the entities.
#[derive(Default, Debug)]
pub struct StringComponentStore {
    components: Components,
    shared: SharedComponents,
}

impl ComponentStore for StringComponentStore {
    type Components = (BuildComponents, BuildSharedComponents);

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
            .filter(|&(k, _)| k.0 == entity)
            .map(|(k, _)| k.clone())
            .collect();

        for k in keys {
            self.components.remove(&k);
        }

        let keys: Vec<(Entity, String)> = self
            .shared
            .iter()
            .filter(|&(k, _)| k.0 == entity)
            .map(|(k, _)| k.clone())
            .collect();

        for k in keys {
            self.shared.remove(&k);
        }
    }

    fn print_entity(&self, entity: impl Into<Entity>) {
        let entity = entity.into();

        println!("Components of entity: {}", entity.0);
        for (k, v) in self.components.iter().filter(|&(k, _)| k.0 == entity) {
            println!("Key: {:?}, Value: {:?}", k, v);
        }

        println!("Shared components of entity: {}", entity.0);
        for (k, v) in self.shared.iter().filter(|&(k, _)| k.0 == entity) {
            println!("Key: {:?}, Value: {:?}", k, v);
        }
    }
}

impl StringComponentStore {
    /// Returns a list of entities that references the same component.
    pub fn entities_of_component(&self, key: impl Into<String>, entity: Entity) -> Vec<Entity> {
        let key = key.into();
        let mut source = entity;
        let mut source_key = key.clone();
        let mut entities = vec![];

        // if a different source is found, entity is not the source
        if let Ok(src) = self.source(entity, key) {
            source = src.0;
            source_key = src.1;
        }

        entities.push(source);

        let mut filtered_entities: Vec<Entity> = self
            .shared
            .iter()
            .filter(|s| (s.1).0 == source && (s.1).1 == source_key)
            .map(|s| (s.0).0)
            .collect();

        entities.append(&mut filtered_entities);

        entities
    }

    /// Register a `component` for the given `entity`.
    pub fn register<C: Component>(&mut self, key: impl Into<String>, entity: Entity, component: C) {
        self.components
            .insert((entity, key.into()), Box::new(component));
    }

    /// Registers a sharing of the given component between the given entities. Uses as source key the component key.
    pub fn register_shared<C: Component>(&mut self, key: &str, target: Entity, source: Entity) {
        self.register_shared_by_source_key::<C>(key, key, target, source);
    }

    /// Registers a sharing of the given component between the given entities.
    pub fn register_shared_by_source_key<C: Component>(
        &mut self,
        key: &str,
        source_key: &str,
        target: Entity,
        source: Entity,
    ) {
        let mut source = source;
        let mut source_key = source_key.to_string();
        if let Ok((src, key)) = self.source(source, source_key.as_str()) {
            source = src;
            source_key = key;
        }
        let target_key = (target, key.to_string());
        self.components.remove(&target_key);
        self.shared.insert(target_key, (source, source_key));
    }

    /// Registers a sharing of the given component between the given entities. Uses as source key the component key.
    pub fn register_shared_box(&mut self, key: &str, target: Entity, source: SharedComponentBox) {
        self.register_shared_box_by_source_key(key, key, target, source);
    }

    /// Registers a sharing of the given component between the given entities.
    pub fn register_shared_box_by_source_key(
        &mut self,
        key: &str,
        source_key: &str,
        target: Entity,
        source: SharedComponentBox,
    ) {
        let mut source = source;
        let mut source_key = source_key.to_string();
        if let Ok((src, key)) = self.source(source.source, source_key.as_str()) {
            source.source = src;
            source_key = key;
        }
        let target_key = (target, key.to_string());
        self.components.remove(&target_key);
        self.shared
            .insert(target_key, (source.source, source_key.to_string()));
    }

    /// Register a `component_box` for the given `entity`.
    pub fn register_box(&mut self, key: &str, entity: Entity, component_box: ComponentBox) {
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
        let key = key.into();
        self.shared
            .get(&(entity, key.clone()))
            .ok_or_else(|| NotFound::Key((entity, key)))
            .map(|s| s.clone())
    }

    /// Returns the target key for a given source and target.
    pub fn target_key(
        &self,
        source: Entity,
        target: Entity,
        source_key: impl Into<String>,
    ) -> Result<String, NotFound> {
        let source_key = (source, source_key.into());

        self.shared
            .iter()
            .find(|(k, v)| k.0 == target && v.0 == source_key.0 && v.1 == source_key.1)
            .ok_or_else(|| NotFound::Key(source_key))
            .map(|(k, _)| k.1.clone())
    }

    /// Returns the source. First search in entities map. If not found search in shared entity map.
    pub fn source(
        &self,
        entity: Entity,
        key: impl Into<String>,
    ) -> Result<(Entity, String), NotFound> {
        let key = (entity, key.into());
        if !self.components.contains_key(&key) {
            let mut source = self.source_from_shared(key.1.clone(), key.0);

            loop {
                if source.is_err() || self.components.contains_key(source.as_ref().unwrap()) {
                    return source;
                }

                source = self.source_from_shared(
                    source.as_ref().unwrap().1.as_str(),
                    source.as_ref().unwrap().0,
                );
            }
        }

        Result::Ok(key)
    }

    /// Check if the given component has the given type.
    pub fn is<C: Component>(&self, key: &str, entity: Entity) -> Result<bool, NotFound> {
        let source = self.source(entity, key);

        match source {
            Ok(source) => self
                .components
                .get(&(source.0, source.1))
                .ok_or_else(|| NotFound::Entity(entity))
                .map(|component| (*component).is::<C>()),
            Err(_) => Result::Err(NotFound::Entity(entity)),
        }
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
    fn entities_of_component() {
        let mut store = StringComponentStore::default();
        let source = Entity::from(1);
        let target = Entity::from(2);
        let target_two = Entity::from(3);
        let target_three = Entity::from(4);
        let target_four = Entity::from(5);
        let component = String::from("Test");

        store.register("test", source, component);
        store.register_shared::<String>("test", target, source);
        store.register_shared_by_source_key::<String>("test_next", "test", target_two, source);
        store.register_shared::<String>("test", target_three, target);
        store.register_shared_by_source_key::<String>(
            "test_next",
            "test_next",
            target_four,
            target_two,
        );

        let entities = store.entities_of_component("test", source);
        assert_eq!(entities.len(), 5);

        let entities = store.entities_of_component("test", target);
        assert_eq!(entities.len(), 5);

        let entities = store.entities_of_component("test_next", target_two);
        assert_eq!(entities.len(), 5);

        let entities = store.entities_of_component("test", target_three);
        assert_eq!(entities.len(), 5);

        let entities = store.entities_of_component("test_next", target_four);
        assert_eq!(entities.len(), 5);

        assert!(entities.contains(&source));
        assert!(entities.contains(&target));
        assert!(entities.contains(&target_two));
        assert!(entities.contains(&target_three));
        assert!(entities.contains(&target_four));
    }

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
        store.register("test", entity, String::from("Test"));
        store.remove_entity(entity);

        assert!(!store.contains_entity(entity));
    }

    #[test]
    fn register() {
        let mut store = StringComponentStore::default();
        let entity = Entity::from(1);
        let component = String::from("Test");

        store.register("test", entity, component);

        assert!(store.get::<String>("test", entity).is_ok());
    }

    #[test]
    fn len() {
        let mut store = StringComponentStore::default();
        let entity = Entity::from(1);

        store.register("string", entity, String::from("Test"));
        store.register("float", entity, 5 as f64);

        assert_eq!(store.len(), 2);
    }

    #[test]
    fn target_key() {
        let mut store = StringComponentStore::default();
        let source = Entity::from(1);
        let target = Entity::from(2);
        let target_two = Entity::from(3);
        let component = String::from("Test");

        store.register("test", source, component);
        store.register_shared_by_source_key::<String>("test_one", "test", target, source);
        store.register_shared_by_source_key::<String>("test_two", "test", target_two, source);

        let result_target = store.target_key(source, target, "test");

        assert!(result_target.is_ok());
        assert_eq!(result_target.unwrap(), "test_one");

        let result_target = store.target_key(source, target_two, "test");

        assert!(result_target.is_ok());
        assert_eq!(result_target.unwrap(), "test_two");
    }

    #[test]
    fn register_shared() {
        let mut store = StringComponentStore::default();
        let source = Entity::from(1);
        let target = Entity::from(2);
        let target_next = Entity::from(3);
        let component = String::from("Test");

        store.register("test", source, component);
        store.register_shared::<String>("test", target, source);
        store.register_shared_by_source_key::<String>("test_next", "test", target_next, source);

        assert!(store.get::<String>("test", source).is_ok());
        assert!(store.get::<String>("test", target).is_ok());
        assert!(store.get::<String>("test_next", target_next).is_ok());
        assert!(store.is_origin::<String>("test", source));
        assert!(!store.is_origin::<String>("test", target));
        assert!(!store.is_origin::<String>("test", target_next));
    }
}
