use dces::prelude::*;

#[derive(Default)]
struct Name {
    value: String,
}

struct StringContext(String);

struct PrintSystem;

impl System<EntityStore, StringContext> for PrintSystem {
    fn run_with_context(
        &self,
        ecm: &mut EntityComponentManager<EntityStore>,
        ctx: &mut StringContext,
    ) {
        let (e_store, c_store) = ecm.stores();

        for entity in &e_store.inner {
            if let Ok(comp) = c_store.get::<Name>("name", *entity) {
                println!("{}", comp.value);
            }
        }

        println!("Context: {}", ctx.0);
    }
}

fn main() {
    let mut world = World::from_entity_store(EntityStore::default());

    world
        .create_entity()
        .components(
            ComponentBuilder::new()
                .with(
                    "name",
                    Name {
                        value: String::from("DCES"),
                    },
                )
                .build(),
        )
        .build();

    world.create_system(PrintSystem).build();
    world.run_with_context(&mut StringContext("I'm the context.".into()));
}
