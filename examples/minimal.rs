use dces::prelude::*;

#[derive(Default)]
struct Name {
    value: String,
}

struct PrintSystem;

impl System<VecEntityStore> for PrintSystem {
    fn run(&self, ecm: &mut EntityComponentManager<VecEntityStore>) {
        let (e_store, c_store) = ecm.stores();

        for entity in &e_store.inner {
            if let Ok(comp) = c_store.borrow_component::<Name>(*entity) {
                println!("{}", comp.value);
            }
        }
    }
}

fn main() {
    let mut world = World::<VecEntityStore>::new();

    world
        .create_entity()
        .with(Name {
            value: String::from("DCES"),
        })
        .build();
    world.create_system(PrintSystem).build();

    world.run();
}
