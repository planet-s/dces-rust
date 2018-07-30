use super::*;

struct TestSystem;

impl System for TestSystem {
    fn run(&self, _entities: &Vec<Entity>, _ecm: &mut EntityComponentManager) {}
} 

#[test]
fn create_entity() {
    let mut world = World::new();
    assert_eq!(0, world.create_entity().build());
    assert_eq!(1, world.create_entity().build());
}

#[test]
fn create_system() {
    let mut world = World::new();
    assert_eq!(0, world.create_system(TestSystem).build());
    assert_eq!(1, world.create_system(TestSystem).build());
}