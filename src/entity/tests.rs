use super::*;

#[derive(Copy, Clone, PartialEq, Debug)]
struct TestComponent;

#[test]
fn test_register_entity() {
    let mut ecm = EntityComponentManager::new();
    ecm.register_entity(5);
    assert!(ecm.entities.contains_key(&5));
    assert_eq!(1, ecm.entities.len());
}

#[test]
fn test_register_component() {
    let mut ecm = EntityComponentManager::new();
    ecm.register_entity(0);
    ecm.register_component(&0, TestComponent);
    assert!(ecm.borrow_component::<TestComponent>(0) == Ok(&TestComponent))
}

#[test]
fn test_build() {
    let eb = EntityBuilder {
        entity: 0,
        entity_component_manager: &mut EntityComponentManager::new(),
    };

    assert_eq!(eb.build(), 0);
}

#[test]
fn test_with() {
    let mut ecm = EntityComponentManager::new();
    ecm.register_entity(0);

    {
        let eb = EntityBuilder {
            entity: 0,
            entity_component_manager: &mut ecm,
        };

        eb.with(TestComponent);
    }

    assert!(ecm.borrow_component::<TestComponent>(0) == Ok(&TestComponent))
}
