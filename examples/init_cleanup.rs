use dces::prelude::*;

struct InitSystem;

impl System<EntityStore, ComponentStore> for InitSystem {
    fn run(&self, _: &mut EntityComponentManager<EntityStore, ComponentStore>) {
        println!("Init");
    }
}

struct CleanupSystem;

impl System<EntityStore, ComponentStore> for CleanupSystem {
    fn run(&self, _: &mut EntityComponentManager<EntityStore, ComponentStore>) {
        println!("Cleanup");
    }
}

struct PrintSystem;

impl System<EntityStore, ComponentStore> for PrintSystem {
    fn run(&self, _: &mut EntityComponentManager<EntityStore, ComponentStore>) {
        println!("Print");
    }
}

fn main() {
    let mut world = World::<EntityStore, ComponentStore>::new();

    world.register_init_system(InitSystem);
    world.create_system(PrintSystem).build();
    world.register_cleanup_system(CleanupSystem);

    world.run();
    world.run();
}
