use dces::prelude::*;

#[derive(Default)]
struct Name { value: String }

struct PrintSystem;

impl System<VecEntityContainer> for PrintSystem {
    fn run(&self, ecm: &mut EntityComponentManager<VecEntityContainer>) {
        for entity in &ecm.entity_container().inner.clone() {
            if let Ok(comp) = ecm.borrow_component::<Name>(*entity) {
                println!("{}", comp.value);
            }
        }
    }
}

fn main() {
    let mut world = World::<VecEntityContainer>::new();

    world.create_entity().with(Name { value: String::from("DCES") }).build();
    world.create_system(PrintSystem).build();

    world.run();
}