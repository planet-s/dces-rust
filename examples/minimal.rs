use dces::prelude::*;

#[derive(Default)]
struct Name {
    value: String,
}

struct PrintSystem;

impl System<EntityStore, ComponentStore, PhantomContext> for PrintSystem {
    fn run(&self, ecm: &mut EntityComponentManager<EntityStore, ComponentStore>) {
        let (e_store, c_store) = ecm.stores();

        for entity in &e_store.inner {
            if let Ok(comp) = c_store.get::<Name>(*entity) {
                println!("{}", comp.value);
            }
        }
    }
}

fn main() {
    let mut world = World::from_stores(EntityStore::default(), ComponentStore::default());

    world
        .create_entity()
        .components(
            ComponentBuilder::new()
                .with(Name {
                    value: String::from("DCES"),
                })
                .build(),
        )
        .build();

    world.create_system(PrintSystem).build();
    world.run();
}
