use std::any::{Any, TypeId};
use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub mod prelude;

pub type Entity = u32;

#[derive(Debug, PartialEq, Eq)]
pub enum NotFound {
    Entity(Entity),
    Component(TypeId),
}

pub trait Component: Any {}

impl<T: Any> Component for T {}

pub trait System: Any {
    fn run(&self, entity_component_manager: &mut EntityComponentManager);
}

pub struct EntitySystem {
    system: Box<System>,
}

pub struct EntityBuilder<'a> {
    pub entity: Entity,

    pub entity_component_manager: &'a mut EntityComponentManager,
}

impl<'a> EntityBuilder<'a> {
    pub fn with<C: Component>(self, component: C) -> Self {
        self.entity_component_manager
            .register_component(&self.entity, component);
        self
    }

    pub fn build(self) -> Entity {
        self.entity
    }
}

pub struct EntitySystemBuilder<'a> {
    pub entity_system_manager: &'a mut EntitySystemManager,
}

impl<'a> EntitySystemBuilder<'a>{
    pub fn with_filter(self) -> Self {
        self
    }

    pub fn with_priority(self) -> Self {
        self
    }

    pub fn build(self) {

    }
}

#[derive(Default)]
pub struct EntitySystemManager {

}

impl EntitySystemManager {
    pub fn new() -> Self {
        Default::default()
    }


}

#[derive(Default)]
pub struct EntityComponentManager {
    entities: HashMap<Entity, HashMap<TypeId, Box<Any>>>,
}

impl EntityComponentManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn register_entity(&mut self, entity: Entity) {
        self.entities.insert(entity, HashMap::new());
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        self.entities.remove(entity);
    }

    pub fn register_component<C: Component>(&mut self, entity: &Entity, component: C) {
        self.entities
            .get_mut(entity)
            .get_or_insert(&mut HashMap::new())
            .insert(TypeId::of::<C>(), Box::new(component));
    }

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
                    })
                    .ok_or_else(|| NotFound::Component(TypeId::of::<C>()))
            })
    }

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
                    })
                    .ok_or_else(|| NotFound::Component(TypeId::of::<C>()))
            })
    }
}

#[derive(Default)]
pub struct World {
    entity_component_manager: EntityComponentManager,
    entity_system_manager: EntitySystemManager,
    entity_counter: u32,
}

impl World {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        let entity = self.entity_counter;
        self.entity_component_manager.register_entity(entity);
        self.entity_counter += 1;
        EntityBuilder {
            entity,
            entity_component_manager: &mut self.entity_component_manager,
        }
    }

    pub fn delete_entity(&mut self, entity: &Entity) {
        self.entity_component_manager.remove_entity(entity);

        // todo: delete entity from all systems
    }

    pub fn create_entity_system<S: System>(&mut self, system: S) -> EntitySystemBuilder {
        EntitySystemBuilder {
            entity_system_manager: &mut self.entity_system_manager,
        }
    }
}
