use dces::prelude::*;

#[derive(Default)]
struct Size {
    width: u32,
    height: u32,
}

#[derive(Default)]
struct Name(String);

#[derive(Default)]
struct Depth(u32);

pub struct SizeSystem {
    source: Entity,
}

impl System<EntityStore, PhantomContext> for SizeSystem {
    fn run(&self, ecm: &mut EntityComponentManager<EntityStore>) {
        if let Ok(comp) = ecm
            .component_store_mut()
            .get_mut::<Size>("size", self.source)
        {
            comp.width += 1;
            comp.height += 1;
        }
    }
}

pub struct PrintSystem;
impl System<EntityStore, PhantomContext> for PrintSystem {
    fn run(&self, ecm: &mut EntityComponentManager<EntityStore>) {
        let (e_store, c_store) = ecm.stores();

        for entity in &e_store.inner {
            if let Ok(name) = c_store.get::<Name>("name", *entity) {
                if let Ok(size) = c_store.get::<Size>("size", *entity) {
                    println!(
                        "entity: {}; name: {}; width: {}; height: {}",
                        entity.0, name.0, size.width, size.height
                    );
                }
            }
        }
    }
}

fn main() {
    let mut world = World::from_entity_store(EntityStore::default());

    let source = world
        .create_entity()
        .components(
            ComponentBuilder::new()
                .with("name", Name(String::from("Button")))
                .with("depth", Depth(4))
                .with(
                    "size",
                    Size {
                        width: 5,
                        height: 5,
                    },
                )
                .build(),
        )
        .build();

    world
        .create_entity()
        .components(
            ComponentBuilder::new()
                .with("name", Name(String::from("CheckBox")))
                .with("depth", Depth(1))
                .with_shared::<Size>("size", source)
                .build(),
        )
        .build();

    world.create_system(PrintSystem).with_priority(1).build();

    world
        .create_system(SizeSystem { source })
        .with_priority(0)
        .build();

    world.run();
}
