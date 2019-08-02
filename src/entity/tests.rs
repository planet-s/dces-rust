use super::*;

#[derive(Copy, Clone, Default, PartialEq, Debug)]
struct TestComponent;

#[test]
fn test_register_entity() {
    let mut ecm = EntityComponentManager::new(VecEntityStore { inner: vec![] });
    ecm.register_entity(5);
    assert!(ecm.component_store().contains_entity(&5.into()));
    assert_eq!(1, ecm.component_store().len());
}

#[test]
fn test_register_component() {
    let mut ecm = EntityComponentManager::new(VecEntityStore { inner: vec![] });
    ecm.register_entity(0);
    ecm.register_component(0.into(), TestComponent);
    assert!(
        ecm.component_store()
            .borrow_component::<TestComponent>(0.into())
            == Ok(&TestComponent)
    )
}

#[test]
fn test_register_shared_component() {
    let mut ecm = EntityComponentManager::new(VecEntityStore { inner: vec![] });
    ecm.register_entity(0);
    ecm.register_component(0.into(), TestComponent);
    ecm.register_entity(1);
    ecm.register_shared_component::<TestComponent>(1.into(), 0.into());
    assert!(
        ecm.component_store()
            .borrow_component::<TestComponent>(1.into())
            == Ok(&TestComponent)
    )
}

#[test]
fn test_build() {
    let eb = EntityBuilder {
        entity: 0.into(),
        entity_component_manager: &mut EntityComponentManager::new(VecEntityStore {
            inner: vec![],
        }),
    };

    assert_eq!(eb.build(), 0.into());
}

#[test]
fn test_with() {
    let mut ecm = EntityComponentManager::new(VecEntityStore { inner: vec![] });
    ecm.register_entity(0);

    {
        let eb = EntityBuilder {
            entity: 0.into(),
            entity_component_manager: &mut ecm,
        };

        eb.with(TestComponent);
    }

    assert!(
        ecm.component_store()
            .borrow_component::<TestComponent>(0.into())
            == Ok(&TestComponent)
    )
}

#[test]
fn test_with_shared() {
    let mut ecm = EntityComponentManager::new(VecEntityStore { inner: vec![] });
    ecm.register_entity(0);

    {
        let eb = EntityBuilder {
            entity: 0.into(),
            entity_component_manager: &mut ecm,
        };

        eb.with(TestComponent);
    }

    ecm.register_entity(1);

    {
        let eb = EntityBuilder {
            entity: 1.into(),
            entity_component_manager: &mut ecm,
        };

        eb.with_shared::<TestComponent>(0.into());
    }

    assert!(
        ecm.component_store()
            .borrow_component::<TestComponent>(1.into())
            == Ok(&TestComponent)
    )
}
