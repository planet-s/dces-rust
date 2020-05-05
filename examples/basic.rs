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

pub struct SizeSystem;
impl System<EntityStore, ComponentStore, DummyContext> for SizeSystem {
    fn run(&self, ecm: &mut EntityComponentManager<EntityStore, ComponentStore>) {
        let (e_store, c_store) = ecm.stores_mut();

        for entity in &e_store.inner {
            if let Ok(comp) = c_store.get_mut::<Size>(*entity) {
                comp.width += 1;
                comp.height += 1;
            }
        }
    }
}

pub struct PrintSystem;
impl System<EntityStore, ComponentStore, DummyContext> for PrintSystem {
    fn run(&self, ecm: &mut EntityComponentManager<EntityStore, ComponentStore>) {
        let (e_store, c_store) = ecm.stores_mut();

        for entity in &e_store.inner {
            if let Ok(name) = c_store.get::<Name>(*entity) {
                if let Ok(size) = c_store.get::<Size>(*entity) {
                    println!("{} width: {}; height: {}", name.0, size.width, size.height);
                }
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
                .with(Name(String::from("Button")))
                .with(Depth(4))
                .with(Size {
                    width: 5,
                    height: 5,
                })
                .build(),
        )
        .build();

    world
        .create_entity()
        .components(
            ComponentBuilder::new()
                .with(Name(String::from("CheckBox")))
                .with(Depth(1))
                .with(Size {
                    width: 3,
                    height: 3,
                })
                .build(),
        )
        .build();

    world
        .create_entity()
        .components(
            ComponentBuilder::new()
                .with(Name(String::from("RadioButton")))
                .with(Depth(2))
                .with(Size {
                    width: 4,
                    height: 6,
                })
                .build(),
        )
        .build();

    world
        .create_entity()
        .components(
            ComponentBuilder::new()
                .with(Depth(3))
                .with(Size {
                    width: 10,
                    height: 4,
                })
                .build(),
        )
        .build();

    world
        .create_entity()
        .components(
            ComponentBuilder::new()
                .with(Depth(0))
                .with(Size {
                    width: 5,
                    height: 8,
                })
                .build(),
        )
        .build();

    world.create_system(PrintSystem).with_priority(1).build();

    world.create_system(SizeSystem).with_priority(0).build();

    world.run();
}
