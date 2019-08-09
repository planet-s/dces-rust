use dces::prelude::*;

struct InitSystem;

impl System<VecEntityStore> for InitSystem {
    fn run(&self, _: &mut EntityComponentManager<VecEntityStore>) {
        println!("Init");
    }
}

struct CleanupSystem;

impl System<VecEntityStore> for CleanupSystem {
    fn run(&self, _: &mut EntityComponentManager<VecEntityStore>) {
        println!("Cleanup");
    }
}

struct PrintSystem;

impl System<VecEntityStore> for PrintSystem {
    fn run(&self, _: &mut EntityComponentManager<VecEntityStore>) {
        println!("Print");
    }
}

fn main() {
    let mut world = World::<VecEntityStore>::new();

    world.register_init_system(InitSystem);
    world.create_system(PrintSystem).build();
    world.register_cleanup_system(CleanupSystem);

    world.run();
    world.run();
}
