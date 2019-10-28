use super::*;

#[derive(Copy, Clone, Default, PartialEq, Debug)]
struct TestComponent;

#[test]
fn test_register_entity() {
    let mut ecm =
        EntityComponentManager::new(VecEntityStore::default(), TypeComponentStore::default());
    ecm.register_entity(5);
    assert!(ecm.component_store().contains_entity(5.into()));
    assert_eq!(1, ecm.component_store().len());
}
