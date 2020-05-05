use dces::prelude::*;

struct InitSystem;

impl System<EntityStore, ComponentStore, DummyContext> for InitSystem {
    fn run(&self, _: &mut EntityComponentManager<EntityStore, ComponentStore>) {
        println!("Init");
    }
}

struct CleanupSystem;

impl System<EntityStore, ComponentStore, DummyContext> for CleanupSystem {
    fn run(&self, _: &mut EntityComponentManager<EntityStore, ComponentStore>) {
        println!("Cleanup");
    }
}

struct PrintSystem;

impl System<EntityStore, ComponentStore, DummyContext> for PrintSystem {
    fn run(&self, _: &mut EntityComponentManager<EntityStore, ComponentStore>) {
        println!("Print");
    }
}

fn main() {
    let mut world = World::from_stores(EntityStore::default(), ComponentStore::default());

    world.register_init_system(InitSystem);
    world.create_system(PrintSystem).build();
    world.register_cleanup_system(CleanupSystem);

    world.run();
    world.run();
}
