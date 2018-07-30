use std::any::{Any, TypeId};
use std::collections::HashMap;

use error::NotFound;

#[cfg(test)]
mod tests;

/// Represents an entity.
pub type Entity = u32;

/// This trait is used to internal handle all components types. This trait is implicitly implemented for all other types.
pub trait Component: Any {}
impl<T: Any> Component for T {}

/// The entity builder is used to create an entity with components.
pub struct EntityBuilder<'a> {
    /// The created entity.
    pub entity: Entity,

    /// Reference to the entity component manager, used to add compoments
    /// to the entity.
    pub entity_component_manager: &'a mut EntityComponentManager,
}

impl<'a> EntityBuilder<'a> {
    /// Add a component of type `C` to the entity.
    pub fn with<C: Component>(self, component: C) -> Self {
        self.entity_component_manager
            .register_component(&self.entity, component);
        self
    }

    /// Finishing the creation of the entity.
    pub fn build(self) -> Entity {
        self.entity
    }
}

/// The EntityComponentManager represents the main entity and component storage.
#[derive(Default)]
pub struct EntityComponentManager {
    /// The entities with its components.
    pub entities: HashMap<Entity, HashMap<TypeId, Box<Any>>>,
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
    pub fn remove_entity(&mut self, entity: &Entity) {
        self.entities.remove(entity);
    }

    /// Register a `component` for the given `entity`.
    pub fn register_component<C: Component>(&mut self, entity: &Entity, component: C) {
        self.entities
            .get_mut(entity)
            .get_or_insert(&mut HashMap::new())
            .insert(TypeId::of::<C>(), Box::new(component));
    }

    /// Returns a refernce of a component of type `C` from the given `entity`. If the entity does
    /// not exists or it dosen't have a component of type `C` `NotFound` will be returned.
    pub fn borrow_component<C: Component>(&self, entity: Entity) -> Result<&C, NotFound> {
        self.entities
            .get(&entity)
            .ok_or_else(|| NotFound::Entity(entity))
            .and_then(|en| {
                en.get(&TypeId::of::<C>())
                    .map(|component| {
                        component.downcast_ref().expect(
                            "EntityComponentManager.borrow_component: internal downcast error",
                        )
                    }).ok_or_else(|| NotFound::Component(TypeId::of::<C>()))
            })
    }

    /// Returns a mutable refernce of a component of type `C` from the given `entity`. If the entity does
    /// not exists or it dosen't have a component of type `C` `NotFound` will be returned.
    pub fn borrow_mut_component<C: Component>(
        &mut self,
        entity: Entity,
    ) -> Result<&mut C, NotFound> {
        self.entities
            .get_mut(&entity)
            .ok_or_else(|| NotFound::Entity(entity))
            .and_then(|en| {
                en.get_mut(&TypeId::of::<C>())
                    .map(|component| {
                        component.downcast_mut().expect(
                            "EntityComponentManager.borrow_mut_component: internal downcast error",
                        )
                    }).ok_or_else(|| NotFound::Component(TypeId::of::<C>()))
            })
    }
}
