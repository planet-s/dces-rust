use dces::prelude::*;

#[derive(Default)]
struct Counter(u32);

struct UpdateSystem;
impl System<VecEntityStore, TypeComponentStore> for UpdateSystem {
    fn run(&self, ecm: &mut EntityComponentManager<VecEntityStore, TypeComponentStore>) {
        let (e_store, c_store) = ecm.stores_mut();

        for entity in &e_store.inner.clone() {
            if let Ok(comp) = c_store.borrow_mut_component::<Counter>(*entity) {
                comp.0 += 1;
            }
        }
    }
}

struct TestUpdateSystem(u32);
impl System<VecEntityStore, TypeComponentStore> for TestUpdateSystem {
    fn run(&self, ecm: &mut EntityComponentManager<VecEntityStore, TypeComponentStore>) {
        let (e_store, c_store) = ecm.stores_mut();

        for entity in &e_store.inner.clone() {
            if let Ok(comp) = c_store.borrow_mut_component::<Counter>(*entity) {
                assert_eq!(comp.0, self.0);
            }
        }
    }
}

#[test]
fn test_update() {
    let mut world = World::<VecEntityStore, TypeComponentStore>::new();

    world
        .create_entity()
        .components(ComponentBuilder::new().with(Counter(0)).build())
        .build();
    world
        .create_entity()
        .components(ComponentBuilder::new().with(Counter(0)).build())
        .build();

    world.create_system(UpdateSystem).with_priority(0).build();
    world
        .create_system(TestUpdateSystem(1))
        .with_priority(1)
        .build();
    world.run();
}
