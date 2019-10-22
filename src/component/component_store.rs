use core::any::{Any, TypeId};

use std::collections::HashMap;

use super::{Component, ComponentBox, ComponentStore, Entity, SharedComponentBox};
use crate::error::NotFound;

/// The `TypeComponentBuilder` is used to build a set of type key based components.
#[derive(Default)]
pub struct TypeComponentBuilder {
    components: HashMap<TypeId, Box<dyn Any>>,
    shared: HashMap<TypeId, Entity>,
}

impl TypeComponentBuilder {
    /// Creates an new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a component of type `C` to the entity.
    pub fn with<C: Component>(mut self, component: C) -> Self {
        self.components
            .insert(TypeId::of::<C>(), Box::new(component));
        self
    }

    /// Adds an entity as `source` for a shared component of type `C`.
    pub fn with_shared<C: Component>(mut self, source: Entity) -> Self {
        self.shared.insert(TypeId::of::<C>(), source);
        self
    }

    /// Adds an entity as `source` for a shared component box.
    pub fn with_shared_box(mut self, source: SharedComponentBox) -> Self {
        self.shared.insert(source.type_id, source.source);
        self
    }

    /// Adds a component box to the entity.
    pub fn with_box(mut self, component_box: ComponentBox) -> Self {
        let (type_id, component) = component_box.consume();
        self.components.insert(type_id, component);
        self
    }

    /// Finishing the creation of the entity.
    pub fn build(self) -> (HashMap<TypeId, Box<dyn Any>>, HashMap<TypeId, Entity>) {
        (self.components, self.shared)
    }
}

/// The `TypeComponentStore` stores the components of all entities. It could be used to
/// borrow the components of the entities.
#[derive(Default, Debug)]
pub struct TypeComponentStore {
    components: HashMap<Entity, HashMap<TypeId, Box<dyn Any>>>,
    shared: HashMap<Entity, HashMap<TypeId, Entity>>,
}

impl ComponentStore for TypeComponentStore {
    type Components = (HashMap<TypeId, Box<dyn Any>>, HashMap<TypeId, Entity>);

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
        let entity = entity.into();
        if !self.components.contains_key(&entity) {
            self.components.insert(entity.into(), HashMap::new());
        }

        if !self.shared.contains_key(&entity) {
            self.shared.insert(entity.into(), HashMap::new());
        }
    }

    fn remove_entity(&mut self, entity: impl Into<Entity>) {
        self.components.remove(&entity.into());
    }
}

impl TypeComponentStore {
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
            self.shared.insert(target, HashMap::new());
        }

        // Removes unshared component of entity.
        if let Some(comp) = self.components.get_mut(&target) {
            comp.remove(&TypeId::of::<C>());
        }

        self.shared
            .get_mut(&target)
            .unwrap()
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
            self.shared.insert(target, HashMap::new());
        }

        // Removes unshared component of entity.
        if let Some(comp) = self.components.get_mut(&target) {
            comp.remove(&source.type_id);
        }

        self.shared
            .get_mut(&target)
            .unwrap()
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

    /// Returns `true` if entity is the origin of the requested component `false`.
    pub fn is_origin<C: Component>(&self, entity: Entity) -> bool {
        if let Some(components) = self.components.get(&entity) {
            return components.contains_key(&TypeId::of::<C>());
        }

        false
    }

    // Search the the target entity in the entity map.
    fn target_entity_from_shared<C: Component>(&self, entity: Entity) -> Result<Entity, NotFound> {
        self.shared
            .get(&entity)
            .ok_or_else(|| NotFound::Entity(entity))
            .and_then(|en| {
                en.get(&TypeId::of::<C>())
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
    pub fn borrow_component<C: Component>(&self, entity: Entity) -> Result<&C, NotFound> {
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
    pub fn borrow_mut_component<C: Component>(
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::TypeId;

    #[test]
    fn builder_with() {
        let builder = TypeComponentBuilder::new();
        let component = String::from("Test");
        let (map, _) = builder.with(component).build();

        assert!(map.contains_key(&TypeId::of::<String>()));
    }

    #[test]
    fn builder_with_shared() {
        let builder = TypeComponentBuilder::new();
        let source = Entity::from(1);
        let (_, map) = builder.with_shared::<String>(source).build();

        assert!(map.contains_key(&TypeId::of::<String>()));
        assert_eq!(*map.get(&TypeId::of::<String>()).unwrap(), source);
    }

    #[test]
    fn builder_with_shared_box() {
        let builder = TypeComponentBuilder::new();
        let source = Entity::from(1);
        let (_, map) = builder
            .with_shared_box(SharedComponentBox::new(TypeId::of::<String>(), source))
            .build();

        assert!(map.contains_key(&TypeId::of::<String>()));
    }

    #[test]
    fn builder_with_box() {
        let builder = TypeComponentBuilder::new();
        let component = String::from("Test");
        let (map, _) = builder.with_box(ComponentBox::new(component)).build();

        assert!(map.contains_key(&TypeId::of::<String>()));
    }

    #[test]
    fn register_entity() {
        let mut store = TypeComponentStore::default();
        let entity = Entity::from(1);
        store.register_entity(entity);

        assert!(store.contains_entity(&entity));
    }

    #[test]
    fn remove_entity() {
        let mut store = TypeComponentStore::default();
        let entity = Entity::from(1);
        store.register_entity(entity);
        store.remove_entity(entity);

        assert!(!store.contains_entity(&entity));
    }

    #[test]
    fn register_component() {
        let mut store = TypeComponentStore::default();
        let entity = Entity::from(1);
        let component = String::from("Test");

        store.register_entity(entity);
        store.register_component(entity, component);

        assert!(store.borrow_component::<String>(entity).is_ok());
    }

    #[test]
    fn len() {
        let mut store = TypeComponentStore::default();
        let entity = Entity::from(1);

        store.register_entity(entity);
        store.register_component(entity, String::from("Test"));
        store.register_component(entity, 5 as f64);

        assert_eq!(store.len(), 1);
    }

    #[test]
    fn register_shared_component() {
        let mut store = TypeComponentStore::default();
        let entity = Entity::from(1);
        let target = Entity::from(2);
        let component = String::from("Test");

        store.register_entity(entity);
        store.register_component(entity, component);
        store.register_shared_component::<String>(target, entity);

        assert!(store.borrow_component::<String>(entity).is_ok());
        assert!(store.borrow_component::<String>(target).is_ok());
        assert!(store.is_origin::<String>(entity));
        assert!(!store.is_origin::<String>(target));
    }

    #[test]
    fn register_component_box() {
        let mut store = TypeComponentStore::default();
        let entity = Entity::from(1);
        let component = String::from("Test");

        store.register_entity(entity);
        store.register_component_box(entity, ComponentBox::new(component));

        assert!(store.borrow_component::<String>(entity).is_ok());
    }

    #[test]
    fn register_shared_component_box() {
        let mut store = TypeComponentStore::default();
        let entity = Entity::from(1);
        let target = Entity::from(2);
        let component = String::from("Test");

        store.register_entity(entity);
        store.register_component(entity, component);
        store.register_shared_component_box(
            target,
            SharedComponentBox::new(TypeId::of::<String>(), entity),
        );
        assert!(store.borrow_component::<String>(entity).is_ok());
        assert!(store.borrow_component::<String>(target).is_ok());
        assert!(store.is_origin::<String>(entity));
        assert!(!store.is_origin::<String>(target));
    }
}
