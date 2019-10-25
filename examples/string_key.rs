use dces::prelude::*;

#[derive(Default)]
struct Name(String);

#[derive(Default)]
struct Depth(u32);

pub struct PrintSystem;
impl System<EntityStore, StringComponentStore> for PrintSystem {
    fn run(&self, ecm: &mut EntityComponentManager<EntityStore, StringComponentStore>) {
        let (e_store, c_store) = ecm.stores();

        for entity in &e_store.inner {
            if let Ok(header) = c_store.get::<String>("header", *entity) {
                println!("{}", header);
            }

            if let Ok(content) = c_store.get::<String>("content", *entity) {
                println!("{}", content);
            }
        }
    }
}

fn main() {
    let mut world = World::from_stores(EntityStore::default(), StringComponentStore::default());

    let source = world
        .create_entity()
        .components(
            StringComponentBuilder::new()
                .with("header", String::from("Header 1"))
                .with("content", String::from("This is the original content."))
                .build(),
        )
        .build();

    world
        .create_entity()
        .components(
            StringComponentBuilder::new()
                .with("header", String::from("Header 2"))
                .with_shared::<String>("content", source)
                .build(),
        )
        .build();

    world.create_system(PrintSystem).with_priority(1).build();

    world.run();
}
