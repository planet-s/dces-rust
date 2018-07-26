use super::*;

#[test]
fn create_entity() {
    let mut world = World::new();
    assert_eq!(0, world.create_entity().build());
    assert_eq!(1, world.create_entity().build());
}
