use super::*;

struct TestSystem;

impl System for TestSystem {
    fn run(&self, _entities: &Vec<Entity>, _ecm: &mut EntityComponentManager) {}
}

#[test]
fn test_register_system() {
    let mut esm = EntitySystemManager::new();
    esm.register_system(TestSystem, 0);

    assert!(esm.entity_systems.contains_key(&0));
}

#[test]
fn test_remove_system() {
    let mut esm = EntitySystemManager::new();
    esm.register_system(TestSystem, 0);
    esm.remove_system(&0);
    
    assert!(!esm.entity_systems.contains_key(&0));
}

#[test]
fn test_register_filter() {
    let mut esm = EntitySystemManager::new();
    esm.register_system(TestSystem, 0);
    esm.register_filter(|_e| {true}, &0);
    
    assert!(esm.entity_systems.get(&0).unwrap().filter.is_some());
}

#[test]
fn test_register_sort() {
    let mut esm = EntitySystemManager::new();
    esm.register_system(TestSystem, 0);
    esm.register_sort(|_e, _c| { None }, &0);
    
    assert!(esm.entity_systems.get(&0).unwrap().sort.is_some());
}

#[test]
fn test_register_priority() {
    let mut esm = EntitySystemManager::new();
    esm.register_system(TestSystem, 0);
    esm.register_priority(&5, &0);
    
    assert_eq!(esm.entity_systems.get(&0).unwrap().priority, 5);
    assert!(esm.priorities.contains_key(&5));
}

#[test]
fn test_borrow_entity_system() {
    let mut esm = EntitySystemManager::new();
    esm.register_system(TestSystem, 0);
    
    assert!(esm.borrow_entity_system(&0).is_ok());
}

#[test]
fn test_borrow_mut_entity_system() {
    let mut esm = EntitySystemManager::new();
    esm.register_system(TestSystem, 0);
    
    assert!(esm.borrow_mut_entity_system(&0).is_ok());
}

#[test]
fn test_remove_entity() {
   let mut es = EntitySystem::new(Box::new(TestSystem));
   es.entities.push(5);
   es.remove_entity(&5);
    
    assert!(!es.entities.contains(&5));
}

#[test]
fn test_with_filter() {
    let mut esm = EntitySystemManager::new();
    esm.register_system(TestSystem, 0);

    {
        let esb = EntitySystemBuilder {
            entity_system_id: 0,
            entity_system_manager: &mut esm, 
            entities: &HashMap::new(),
        };

        esb.with_filter(|_e| {true});
    }

    assert!(esm.entity_systems.get(&0).unwrap().filter.is_some());
}

#[test]
fn test_with_sort() {
    let mut esm = EntitySystemManager::new();
    esm.register_system(TestSystem, 0);

    {
        let esb = EntitySystemBuilder {
            entity_system_id: 0,
            entity_system_manager: &mut esm, 
            entities: &HashMap::new(),
        };

        esb.with_sort(|_e, _c| { None });
    }

    assert!(esm.entity_systems.get(&0).unwrap().sort.is_some());
}

#[test]
fn test_build() {
    let mut esm = EntitySystemManager::new();
    esm.register_system(TestSystem, 0);

    {
        let esb = EntitySystemBuilder {
            entity_system_id: 0,
            entity_system_manager: &mut esm, 
            entities: &HashMap::new(),
        };

         assert_eq!(esb.build(), 0);
    }
}