use dces::prelude::*;

struct InitSystem;

impl System<VecEntityContainer> for InitSystem {
    fn run(&self, _: &mut EntityComponentManager<VecEntityContainer>) {
        println!("Init");
    }
}

struct CleanupSystem;

impl System<VecEntityContainer> for CleanupSystem {
    fn run(&self, _: &mut EntityComponentManager<VecEntityContainer>) {
        println!("Cleanup");
    }
}

struct PrintSystem;

impl System<VecEntityContainer> for PrintSystem {
    fn run(&self, _: &mut EntityComponentManager<VecEntityContainer>) {
        println!("Print");
    }
}

fn main() {
    let mut world = World::<VecEntityContainer>::new();

    world.register_init_system(InitSystem);
    world.create_system(PrintSystem).build();
    world.register_cleanup_system(CleanupSystem);

    world.run();
    world.run();
}