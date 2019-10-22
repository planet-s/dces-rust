use dces::prelude::*;

struct InitSystem;

impl System<VecEntityStore, TypeComponentStore> for InitSystem {
    fn run(&self, _: &mut EntityComponentManager<VecEntityStore, TypeComponentStore>) {
        println!("Init");
    }
}

struct CleanupSystem;

impl System<VecEntityStore, TypeComponentStore> for CleanupSystem {
    fn run(&self, _: &mut EntityComponentManager<VecEntityStore, TypeComponentStore>) {
        println!("Cleanup");
    }
}

struct PrintSystem;

impl System<VecEntityStore, TypeComponentStore> for PrintSystem {
    fn run(&self, _: &mut EntityComponentManager<VecEntityStore, TypeComponentStore>) {
        println!("Print");
    }
}

fn main() {
    let mut world = World::<VecEntityStore, TypeComponentStore>::new();

    world.register_init_system(InitSystem);
    world.create_system(PrintSystem).build();
    world.register_cleanup_system(CleanupSystem);

    world.run();
    world.run();
}
