extern crate dces;

use dces::prelude::*;

struct Counter(u32);

struct UpdateSystem;
impl System for UpdateSystem {
    fn run(&self, entities: &Vec<Entity>, ecm: &mut EntityComponentManager) {
        for entity in entities {
            if let Ok(comp) = ecm.borrow_mut_component::<Counter>(*entity) {
                comp.0 += 1;
            }
        }
    }
}

struct TestUpdateSystem(u32);
impl System for TestUpdateSystem {
    fn run(&self, entities: &Vec<Entity>, ecm: &mut EntityComponentManager) {
        for entity in entities {
            if let Ok(comp) = ecm.borrow_mut_component::<Counter>(*entity) {
                assert_eq!(comp.0, self.0);
            }
        }
    }
}

#[test]
fn test_update() {
    let mut world = World::new();

    world.create_entity().with(Counter(0)).build();
    world.create_entity().with(Counter(0)).build();

    world.create_system(UpdateSystem).with_priority(&0).build();
    world
        .create_system(TestUpdateSystem(1))
        .with_priority(&1)
        .build();
    world.run();
}
