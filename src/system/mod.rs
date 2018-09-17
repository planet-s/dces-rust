use std::any::{Any, TypeId};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::cell::Cell;

use entity::{Entity, EntityComponentManager};
use error::NotFound;

#[cfg(test)]
mod tests;

/// The run order of a system. The systems will be excuted by priority from small to great.
pub type Priority = i32;

/// This trait is used to interact with the components of entities. It could
/// read and write to the components.
pub trait System: Any {
    fn run(&self, entities: &Vec<Entity>, ecm: &mut EntityComponentManager);
}

/// Internal wrapper for a system. Contains also filter, priority, sort and entities.
pub struct EntitySystem {
    /// The wrapped system.
    pub system: Box<System>,

    /// List of filtered / sorted system entities.
    pub entities: Vec<Entity>,
    filter: Option<Arc<Fn(Vec<&Box<Any>>) -> bool>>,
    priority: Priority,
    sort: Option<Arc<Fn(&Box<Any>, &Box<Any>) -> Option<Ordering>>>,
}

impl EntitySystem {
    /// Create a new entity system.
    pub fn new(system: Box<System>) -> Self {
        EntitySystem {
            system,
            filter: None,
            priority: 0,
            entities: vec![],
            sort: None,
        }
    }

    /// Apply the system filter and sort on the given `entities`.
    pub fn apply_filter_and_sort(&mut self, entities: &HashMap<Entity, HashMap<TypeId, Box<Any>>>) {
        self.entities.clear();

        let entity_iterator = entities.iter();

        // filter entities by systems filter closure
        if let Some(ref f) = self.filter {
            let filter = f.clone();

            let filtered_entities: Vec<Entity> = entity_iterator
                .filter(|&(_, v)| filter(v.iter().map(|(_, cv)| cv).collect()))
                .map(|(k, _)| *k)
                .collect();

            self.entities.extend(filtered_entities);
        } else {
            let all_entites: Vec<Entity> = entity_iterator.map(|(key, _)| *key).collect();
            self.entities.extend(all_entites);
        }

        // sort entities by systems filter closure
        if let Some(ref s) = self.sort {
            let sort = s.clone();

            self.entities.sort_by(|a, b| {
                for (_, comp_a) in entities.get(a).unwrap() {
                    for (_, comp_b) in entities.get(b).unwrap() {
                        if let Some(ord) = sort(&comp_a, &comp_b) {
                            return ord;
                        }
                    }
                }

                Ordering::Equal
            });
        }
    }

    /// Remove the given `entity` from the system.
    pub fn remove_entity(&mut self, entity: Entity) {
        if !self.entities.contains(&entity) {
            return;
        }

        let mut remove_index = 0;
        for i in 0..self.entities.len() {
            if self.entities[i] == entity {
                remove_index = i;
                break;
            }
        }

        self.entities.remove(remove_index);
    }
}

/// The entity system builder is used to create an entity system.
pub struct EntitySystemBuilder<'a> {
    /// Id of the entity system.
    pub entity_system_id: u32,

    /// Reference to the entity system manager, used to apply filter, sort and priority
    /// to the system.
    pub entity_system_manager: &'a mut EntitySystemManager,

    /// List of entities. Used to generate a filterd and sorted list of entities per system.
    pub entities: &'a HashMap<Entity, HashMap<TypeId, Box<Any>>>,

    pub priority: Cell<i32>,
}

impl<'a> EntitySystemBuilder<'a> {
    /// Add a `filter` to the system.
    pub fn with_filter<F>(self, filter: F) -> Self
    where
        F: Fn(Vec<&Box<Any>>) -> bool + 'static,
    {
        self.entity_system_manager
            .register_filter(filter, self.entity_system_id);
        self
    }

    /// Add a `priority` to the system. Default priority is 0.
    pub fn with_priority(self, priority: Priority) -> Self {
        self.priority.set(priority);
        self
    }

    /// Add a `sort` to the system.
    pub fn with_sort<S>(self, sort: S) -> Self
    where
        S: Fn(&Box<Any>, &Box<Any>) -> Option<Ordering> + 'static,
    {
        self.entity_system_manager
            .register_sort(sort, self.entity_system_id);
        self
    }

    /// Finishing the creation of the system.
    pub fn build(self) -> u32 {
        println!("Sys: {}, Prio: {} ",self.entity_system_id, self.priority.get());
        self.entity_system_manager.register_priority(self.priority.get(), self.entity_system_id);
        let entity_system = self
            .entity_system_manager
            .borrow_mut_entity_system(self.entity_system_id)
            .unwrap();

        entity_system.apply_filter_and_sort(self.entities);

        self.entity_system_id
    }
}

/// The EntitySystemManager represents the main system storage.
#[derive(Default)]
pub struct EntitySystemManager {
    /// The entity systems.
    entity_systems: HashMap<u32, EntitySystem>,

    /// Priorities of the systems.
    pub priorities: BTreeMap<i32, Vec<u32>>,
}

impl EntitySystemManager {
    /// Creates a new entity system manager.
    pub fn new() -> Self {
        Default::default()
    }

    /// Register a new `system`.
    pub fn register_system<S: System>(&mut self, system: S, system_id: u32) {
        self.entity_systems
            .insert(system_id, EntitySystem::new(Box::new(system)));
    }

    /// Removes a system from the storage.
    pub fn remove_system(&mut self, system_id: u32) {
        self.entity_systems.remove(&system_id);
    }

    /// Register a `filter` for the system with the given `system_id`.
    pub fn register_filter<F>(&mut self, filter: F, system_id: u32)
    where
        F: Fn(Vec<&Box<Any>>) -> bool + 'static,
    {
        self.entity_systems.get_mut(&system_id).unwrap().filter = Some(Arc::new(filter));
    }

    /// Register a `priority` for the system with the given `system_id`.
    pub fn register_priority(&mut self, priority: Priority, system_id: u32) {
        self.entity_systems.get_mut(&system_id).unwrap().priority = priority;

        // insert new priority and add system to it.
        if !self.priorities.contains_key(&priority) {
            self.priorities.insert(priority, vec![system_id]);
            return
        }

        self.priorities.get_mut(&priority).unwrap().push(system_id);
    }

    /// Register a `sort` for the system with the given `system_id`.
    pub fn register_sort<S>(&mut self, sort: S, system_id: u32)
    where
        S: Fn(&Box<Any>, &Box<Any>) -> Option<Ordering> + 'static,
    {
        self.entity_systems.get_mut(&system_id).unwrap().sort = Some(Arc::new(sort));
    }

    /// Removes the give `entity` from all systems.
    pub fn remove_entity(&mut self, entity: Entity) {
        for (_, es) in &mut self.entity_systems {
            es.remove_entity(entity);
        }
    }

    /// Filters and sort the entities of a system.
    pub fn apply_filter_and_sort(&mut self, entities: &HashMap<Entity, HashMap<TypeId, Box<Any>>>) {
        for (_, es) in &mut self.entity_systems {
            es.apply_filter_and_sort(entities);
        }
    }

    /// Returns a refernce of a entity system. If the entity system does not exists `NotFound` will be returned.
    pub fn borrow_entity_system(&self, entity_system_id: u32) -> Result<&EntitySystem, NotFound> {
        self.entity_systems.get(&entity_system_id).map_or_else(
            || Err(NotFound::EntitySystem(entity_system_id)),
            |es| Ok(es),
        )
    }

    /// Returns a mutable refernce of a entity system. If the entity system does not exists `NotFound` will be returned.
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
