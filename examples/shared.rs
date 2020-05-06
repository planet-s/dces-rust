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

impl System<EntityStore, ComponentStore, PhantomContext> for SizeSystem {
    fn run(&self, ecm: &mut EntityComponentManager<EntityStore, ComponentStore>) {
        if let Ok(comp) = ecm.component_store_mut().get_mut::<Size>(self.source) {
            comp.width += 1;
            comp.height += 1;
        }
    }
}

pub struct PrintSystem;
impl System<EntityStore, ComponentStore, PhantomContext> for PrintSystem {
    fn run(&self, ecm: &mut EntityComponentManager<EntityStore, ComponentStore>) {
        let (e_store, c_store) = ecm.stores();

        for entity in &e_store.inner {
            if let Ok(name) = c_store.get::<Name>(*entity) {
                if let Ok(size) = c_store.get::<Size>(*entity) {
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
    let mut world = World::from_stores(EntityStore::default(), ComponentStore::default());

    let source = world
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
                .with_shared::<Size>(source)
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
