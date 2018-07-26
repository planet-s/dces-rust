use std::any::{Any, TypeId};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(test)]
mod tests;

pub mod prelude;

pub type Entity = u32;
pub type Priority = i32;

#[derive(Debug, PartialEq, Eq)]
pub enum NotFound {
    Entity(Entity),
    Component(TypeId),
    EntitySystem(u32),
}

pub trait Component: Any {}

impl<T: Any> Component for T {}

pub trait System: Any {
    fn run(&self, entity_component_manager: &mut EntityComponentManager);
}

pub struct EntitySystem {
    system: Box<System>,
    filter: Option<Arc<Fn(Vec<&Box<Any>>) -> bool>>,
    priority: Priority,
    entities: Vec<Entity>,
    sort: Option<Arc<Fn(&Box<Any>, &Box<Any>) -> Ordering>>,
}

impl EntitySystem {
    pub fn new(system: Box<System>) -> Self {
        EntitySystem {
            system,
            filter: None,
            priority: 0,
            entities: vec![],
            sort: None,
        }
    }

    pub fn apply_filter_and_sort(&mut self, entities: &HashMap<Entity, HashMap<TypeId, Box<Any>>>) {
        self.entities.clear();

        if let Some(ref f) = self.filter {
            let filter = f.clone();
            let entity_iterator = entities.iter();

            let filtered_entities: Vec<u32> = entity_iterator
                .filter(|&(_, v)| filter(v.iter().map(|(_, cv)| cv).collect()))
                .map(|(k, _)| *k)
                .collect();

            self.entities.extend(filtered_entities);
        }

    // todo: also sort
        if let Some(ref s) = self.sort {
            let sort = s.clone();
            let entity_iterator = entities.iter();

            // let filtered_entities : HashMap<Entity, HashMap<TypeId, Box<Any>>> = entity_iterator.filter(|&(k, _)| {
            //     self.entities.contains(k)
            // }).collect();
            
            // self.entities.sort_by(|a, b| {

            // })
        }

        
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(entity as usize);
    }
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
    pub entity_system_id: u32,

    pub entity_system_manager: &'a mut EntitySystemManager,

    pub entity_component_manager: &'a mut EntityComponentManager,
}

impl<'a> EntitySystemBuilder<'a> {
    pub fn with_filter<F>(self, filter: F) -> Self
    where
        F: Fn(Vec<&Box<Any>>) -> bool + 'static,
    {
        self.entity_system_manager
            .register_filter(filter, self.entity_system_id);
        self
    }

    pub fn with_priority(self, priority: Priority) -> Self {
        self.entity_system_manager
            .register_priority(priority, self.entity_system_id);
        self
    }

    pub fn with_sort<S>(self, sort: S) -> Self
    where
        S: Fn(&Box<Any>, &Box<Any>) -> Ordering + 'static,
    {
        self.entity_system_manager
            .register_sort(sort, self.entity_system_id);
        self
    }

    pub fn build(self) -> u32 {
        let entity_system = self
            .entity_system_manager
            .borrow_mut_entity_system(self.entity_system_id)
            .unwrap();

        entity_system.apply_filter_and_sort(&self.entity_component_manager.entities);

        // todo filtering should also run after new entiy is added!!!
        // todo implement entity sort for systems

        self.entity_system_id
    }
}

#[derive(Default)]
pub struct EntitySystemManager {
    entity_systems: HashMap<u32, EntitySystem>,
}

impl EntitySystemManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn register_system<S: System>(&mut self, system: S, system_id: u32) {
        self.entity_systems
            .insert(system_id, EntitySystem::new(Box::new(system)));
    }

    pub fn remove_system(&mut self, system_id: &u32) {
        self.entity_systems.remove(system_id);
    }

    pub fn register_filter<F>(&mut self, filter: F, system_id: u32)
    where
        F: Fn(Vec<&Box<Any>>) -> bool + 'static,
    {
        self.entity_systems.get_mut(&system_id).unwrap().filter = Some(Arc::new(filter));
    }

    pub fn register_priority(&mut self, priority: Priority, system_id: u32) {
        self.entity_systems.get_mut(&system_id).unwrap().priority = priority;
    }

    pub fn register_sort<S>(&mut self, sort: S, system_id: u32)
    where
        S: Fn(&Box<Any>, &Box<Any>) -> Ordering + 'static,
    {
        self.entity_systems.get_mut(&system_id).unwrap().sort = Some(Arc::new(sort));
    }

    pub fn borrow_entity_system(&self, entity_system_id: u32) -> Result<&EntitySystem, NotFound> {
        self.entity_systems.get(&entity_system_id).map_or_else(
            || Err(NotFound::EntitySystem(entity_system_id)),
            |es| Ok(es),
        )
    }

    pub fn borrow_mut_entity_system(
        &mut self,
        entity_system_id: u32,
    ) -> Result<&mut EntitySystem, NotFound> {
        self.entity_systems.get_mut(&entity_system_id).map_or_else(
            || Err(NotFound::EntitySystem(entity_system_id)),
            |es| Ok(es),
        )
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

        // todo use this as system filter
        //        let blub : Vec<u32> = self.entities.iter().filter(|&(k, v)| *k == 1 ).map(|(k, v)| *k).collect();
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
    entity_sytem_counter: u32,
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

        for (_, es) in &mut self.entity_system_manager.entity_systems {
            es.remove_entity(*entity);
        }
    }

    pub fn delete_system(&mut self, system_id: &u32) {
        self.entity_system_manager.remove_system(system_id);
    }

    pub fn create_system<S: System>(&mut self, system: S) -> EntitySystemBuilder {
        let entity_system_id = self.entity_sytem_counter;
        self.entity_system_manager
            .register_system(system, entity_system_id);
        self.entity_sytem_counter += 1;

        EntitySystemBuilder {
            entity_system_manager: &mut self.entity_system_manager,
            entity_component_manager: &mut self.entity_component_manager,
            entity_system_id,
        }
    }
}
