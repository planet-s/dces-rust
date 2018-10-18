use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;

use error::NotFound;

#[cfg(test)]
mod tests;

/// Represents an entity.
pub type Entity = u32;

/// This trait is used to internal handle all components types. This trait is implicitly implemented for all other types.
pub trait Component: Any {}
impl<T: Any> Component for T {}

pub struct ComponentBox {
    component: Box<Any>,
    type_id: TypeId,
}

/// This sturct is used to store a component with its type id. Used for dynamic compement adding.
impl ComponentBox {
    /// Creates the component box.
    pub fn new<C: Component>(component: C) -> Self {
        ComponentBox {
            component: Box::new(component),
            type_id: TypeId::of::<C>(),
        }
    }

    /// Consumes the component box and returns the type id and the component.
    pub fn consume(self) -> (TypeId, Box<Any>) {
        (self.type_id, self.component)
    }
}

/// The entity builder is used to create an entity with components.
pub struct EntityBuilder<'a, T>
where
    T: EntityContainer + 'a,
{
    /// The created entity.
    pub entity: Entity,

    /// Reference to the entity component manager, used to add compoments
    /// to the entity.
    pub entity_component_manager: &'a mut EntityComponentManager,

    pub entity_container: &'a mut T,
}

impl<'a, T> EntityBuilder<'a, T>
where
    T: EntityContainer,
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

    /// Adds a component box to the entity.
    pub fn with_box(self, component_box: ComponentBox) -> Self {
        self.entity_component_manager
            .register_component_box(self.entity, component_box);
        self
    }

    /// Finishing the creation of the entity.
    pub fn build(self) -> Entity {
        self.entity_container.register_entity(self.entity);
        self.entity
    }
}

/// The EntityComponentManager represents the main entity and component storage.
#[derive(Default)]
pub struct EntityComponentManager {
    /// The entities with its components.
    pub entities: HashMap<Entity, HashMap<TypeId, Box<Any>>>,

    pub shared: HashMap<Entity, RefCell<HashMap<TypeId, Entity>>>,
}

impl EntityComponentManager {
    /// Create sa new entity component manager.
    pub fn new() -> Self {
        Default::default()
    }

    /// Register a new `entity`.
    pub fn register_entity(&mut self, entity: Entity) {
        self.entities.insert(entity, HashMap::new());
    }

    /// Removes a `entity` from the manager.
    pub fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    /// Register a `component` for the given `entity`.
    pub fn register_component<C: Component>(&mut self, entity: Entity, component: C) {
        self.entities
            .get_mut(&entity)
            .get_or_insert(&mut HashMap::new())
            .insert(TypeId::of::<C>(), Box::new(component));
    }

    pub fn register_shared_component<C: Component>(&mut self, target: Entity, source: Entity) {
        if !self.shared.contains_key(&target) {
            self.shared.insert(target, RefCell::new(HashMap::new()));
        }

        self.shared[&target]
            .borrow_mut()
            .insert(TypeId::of::<C>(), source);
    }

    /// Register a `component_box` for the given `entity`.
    pub fn register_component_box(&mut self, entity: Entity, component_box: ComponentBox) {
        let (type_id, component) = component_box.consume();

        self.entities
            .get_mut(&entity)
            .get_or_insert(&mut HashMap::new())
            .insert(type_id, component);
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
        if !self.entities.contains_key(&entity)
            || !self.entities[&entity].contains_key(&TypeId::of::<C>())
        {
            return self.target_entity_from_shared::<C>(entity);
        }

        Result::Ok(entity)
    }

    /// Returns a refernce of a component of type `C` from the given `entity`. If the entity does
    /// not exists or it dosen't have a component of type `C` `NotFound` will be returned.
    pub fn borrow_component<C: Component>(&self, entity: Entity) -> Result<&C, NotFound> {
        let target_entity = self.target_entity::<C>(entity);

        match target_entity {
            Ok(entity) => self
                .entities
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

    /// Returns a mutable refernce of a component of type `C` from the given `entity`. If the entity does
    /// not exists or it dosen't have a component of type `C` `NotFound` will be returned.
    pub fn borrow_mut_component<C: Component>(
        &mut self,
        entity: Entity,
    ) -> Result<&mut C, NotFound> {
        let target_entity = self.target_entity::<C>(entity);

        match target_entity {
            Ok(entity) => self
                .entities
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

/// This trait is used to define a custom container for entities.
/// A entity container is used for entiy iteration inside of the
/// system's run methods.
pub trait EntityContainer {
    /// Registers the give 'entity'.
    fn register_entity(&mut self, entity: Entity);

    /// Removes the given 'entity'.
    fn remove_entity(&mut self, entity: Entity);
}

/// VecEntityContainer is the default vector based implementation of an entiy container.
#[derive(Default)]
pub struct VecEntityContainer {
    pub inner: Vec<Entity>,
}

impl EntityContainer for VecEntityContainer {
    fn register_entity(&mut self, entity: Entity) {
        self.inner.push(entity);
    }

    fn remove_entity(&mut self, entity: Entity) {
        self.inner
            .iter()
            .position(|&n| n == entity)
            .map(|e| self.inner.remove(e));
    }
}
