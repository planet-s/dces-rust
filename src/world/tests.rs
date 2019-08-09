use super::*;

#[derive(Default)]
struct TestSystem;

impl System<VecEntityStore> for TestSystem {
    fn run(&self, _ecm: &mut EntityComponentManager<VecEntityStore>) {}
}

#[test]
fn create_entity() {
    let mut world = World::<VecEntityStore>::new();
    assert_eq!(Entity(0), world.create_entity().build());
    assert_eq!(Entity(1), world.create_entity().build());
}

#[test]
fn create_system() {
    let mut world = World::<VecEntityStore>::new();
    assert_eq!(0, world.create_system(TestSystem).build());
    assert_eq!(1, world.create_system(TestSystem).build());
}
