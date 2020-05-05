use dces::prelude::*;

#[derive(Default)]
struct Counter(u32);

struct UpdateSystem;
impl System<EntityStore, ComponentStore, DummyContext> for UpdateSystem {
    fn run(&self, ecm: &mut EntityComponentManager<EntityStore, ComponentStore>) {
        let (e_store, c_store) = ecm.stores_mut();

        for entity in &e_store.inner.clone() {
            if let Ok(comp) = c_store.get_mut::<Counter>(*entity) {
                comp.0 += 1;
            }
        }
    }
}

struct TestUpdateSystem(u32);
impl System<EntityStore, ComponentStore, DummyContext> for TestUpdateSystem {
    fn run(&self, ecm: &mut EntityComponentManager<EntityStore, ComponentStore>) {
        let (e_store, c_store) = ecm.stores_mut();

        for entity in &e_store.inner.clone() {
            if let Ok(comp) = c_store.get_mut::<Counter>(*entity) {
                assert_eq!(comp.0, self.0);
            }
        }
    }
}

#[test]
fn test_update() {
    let mut world = World::from_stores(EntityStore::default(), ComponentStore::default());

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
