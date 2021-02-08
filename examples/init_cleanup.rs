use dces::prelude::*;

struct InitSystem;

impl System<EntityStore> for InitSystem {
    fn run(&self, _: &mut EntityComponentManager<EntityStore>, _: &mut Resources) {
        println!("Init");
    }
}

struct CleanupSystem;

impl System<EntityStore> for CleanupSystem {
    fn run(&self, _: &mut EntityComponentManager<EntityStore>, _: &mut Resources) {
        println!("Cleanup");
    }
}

struct PrintSystem;

impl System<EntityStore> for PrintSystem {
    fn run(&self, _: &mut EntityComponentManager<EntityStore>, _: &mut Resources) {
        println!("Print");
    }
}

fn main() {
    let mut world = World::from_entity_store(EntityStore::default());

    world.register_init_system(InitSystem);
    world.create_system(PrintSystem).build();
    world.register_cleanup_system(CleanupSystem);

    world.run();
    world.run();
}
