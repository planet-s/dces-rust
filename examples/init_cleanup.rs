use dces::prelude::*;

struct InitSystem;

impl System<EntityStore, PhantomContext> for InitSystem {
    fn run(&self, _: &mut EntityComponentManager<EntityStore>) {
        println!("Init");
    }
}

struct CleanupSystem;

impl System<EntityStore, PhantomContext> for CleanupSystem {
    fn run(&self, _: &mut EntityComponentManager<EntityStore>) {
        println!("Cleanup");
    }
}

struct PrintSystem;

impl System<EntityStore, PhantomContext> for PrintSystem {
    fn run(&self, _: &mut EntityComponentManager<EntityStore>) {
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
