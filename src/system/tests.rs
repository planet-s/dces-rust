use crate::entity::VecEntityContainer;
use super::*;

struct TestSystem;

impl System<VecEntityContainer> for TestSystem {
    fn run(&self, _entities: &VecEntityContainer, _ecm: &mut EntityComponentManager) {}
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
    esm.remove_system(0);
    
    assert!(!esm.entity_systems.contains_key(&0));
}

#[test]
fn test_register_priority() {
    let mut esm = EntitySystemManager::new();
    esm.register_system(TestSystem, 0);
    esm.register_priority(5, 0);
    
    assert_eq!(esm.entity_systems.get(&0).unwrap().priority, 5);
    assert!(esm.priorities.contains_key(&5));
}

#[test]
fn test_borrow_entity_system() {
    let mut esm = EntitySystemManager::new();
    esm.register_system(TestSystem, 0);
    
    assert!(esm.borrow_entity_system(0).is_ok());
}

#[test]
fn test_borrow_mut_entity_system() {
    let mut esm = EntitySystemManager::new();
    esm.register_system(TestSystem, 0);
    
    assert!(esm.borrow_mut_entity_system(0).is_ok());
}

#[test]
fn test_build() {
    let mut esm = EntitySystemManager::new();
    esm.register_system(TestSystem, 0);

    {
        let esb = EntitySystemBuilder {
            entity_system_id: 0,
            entity_system_manager: &mut esm, 
            priority: Cell::new(0),
        };

         assert_eq!(esb.build(), 0);
    }
}