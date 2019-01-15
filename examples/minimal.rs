extern crate dces;

use dces::prelude::*;

struct Size {
    width: u32,
    height: u32,
}

struct Name(String);

struct Depth(u32);

pub struct SizeSystem;
impl System<VecEntityContainer> for SizeSystem {
    fn run(&self, entities: &VecEntityContainer, ecm: &mut EntityComponentManager) {
        for entity in &entities.inner {
            if let Ok(comp) = ecm.borrow_mut_component::<Size>(*entity) {
                comp.width += 1;
                comp.height += 1;
            }
        }
    }
}

pub struct PrintSystem;
impl System<VecEntityContainer> for PrintSystem {
    fn run(&self, entities: &VecEntityContainer, ecm: &mut EntityComponentManager) {
        for entity in &entities.inner {
            if let Ok(name) = ecm.borrow_component::<Name>(*entity) {
                if let Ok(size) = ecm.borrow_component::<Size>(*entity) {
                    println!("{} width: {}; height: {}", name.0, size.width, size.height);
                }
            }
        }
    }
}

fn main() {
    let mut world = World::<VecEntityContainer>::new();

    world
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
        .with(Size {
            width: 3,
            height: 3,
        })
        .build();

    world
        .create_entity()
        .with(Name(String::from("RadioButton")))
        .with(Depth(2))
        .with(Size {
            width: 4,
            height: 6,
        })
        .build();

    world
        .create_entity()
        .with(Depth(3))
        .with(Size {
            width: 10,
            height: 4,
        })
        .build();

    world
        .create_entity()
        .with(Depth(0))
        .with(Size {
            width: 5,
            height: 8,
        })
        .build();

    world.create_system(PrintSystem).with_priority(1).build();

    world.create_system(SizeSystem).with_priority(0).build();

    world.run();
}
