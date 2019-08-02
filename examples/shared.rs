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

impl System<VecEntityContainer> for SizeSystem {
    fn run(&self, ecm: &mut EntityComponentManager<VecEntityContainer>) {
        if let Ok(comp) = ecm.borrow_mut_component::<Size>(self.source) {
            comp.width += 1;
            comp.height += 1;
        }
    }
}

pub struct PrintSystem;
impl System<VecEntityContainer> for PrintSystem {
    fn run(&self, ecm: &mut EntityComponentManager<VecEntityContainer>) {
        for entity in &ecm.entity_container().inner.clone() {
            if let Ok(name) = ecm.borrow_component::<Name>(*entity) {
                if let Ok(size) = ecm.borrow_component::<Size>(*entity) {
                    println!("entity: {}; name: {}; width: {}; height: {}", entity.0, name.0, size.width, size.height);
                }
            }
        }
    }
}

fn main() {
    let mut world = World::<VecEntityContainer>::new();

    let source = world
        .create_entity()
        .with(Name(String::from("Button")))
        .with(Depth(4))
        .with(Size {
            width: 5,
            height: 5,
        })
        .build();

    world
        .create_entity()
        .with(Name(String::from("CheckBox")))
        .with(Depth(1))
        .with_shared::<Size>(source)
        .build();

    world.create_system(PrintSystem).with_priority(1).build();

    world
        .create_system(SizeSystem { source })
        .with_priority(0)
        .build();

    world.run();
}
