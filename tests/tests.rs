use dces::prelude::*;

#[derive(Default)]
struct Counter(u32);

struct UpdateSystem;
impl System<EntityStore, PhantomContext> for UpdateSystem {
    fn run(&self, ecm: &mut EntityComponentManager<EntityStore>) {
        let (e_store, c_store) = ecm.stores_mut();

        for entity in &e_store.inner.clone() {
            if let Ok(comp) = c_store.get_mut::<Counter>("counter", *entity) {
                comp.0 += 1;
            }
        }
    }
}

struct TestUpdateSystem(u32);
impl System<EntityStore, PhantomContext> for TestUpdateSystem {
    fn run(&self, ecm: &mut EntityComponentManager<EntityStore>) {
        let (e_store, c_store) = ecm.stores_mut();

        for entity in &e_store.inner.clone() {
            if let Ok(comp) = c_store.get_mut::<Counter>("counter", *entity) {
                assert_eq!(comp.0, self.0);
            }
        }
    }
}

#[test]
fn test_update() {
    let mut world = World::from_entity_store(EntityStore::default());

    world
        .create_entity()
        .components(ComponentBuilder::new().with("counter", Counter(0)).build())
        .build();
    world
        .create_entity()
        .components(ComponentBuilder::new().with("counter", Counter(0)).build())
        .build();

    world.create_system(UpdateSystem).with_priority(0).build();
    world
        .create_system(TestUpdateSystem(1))
        .with_priority(1)
        .build();
    world.run();
}
