extern crate dces;

use std::cmp::Ordering;

use dces::prelude::*;

pub struct Size;

pub struct Background;

pub struct Selector;

pub struct PrintSystem {}

pub struct Depth {
    value: u32,
}

impl System for PrintSystem {
    fn run(&self, entity_component_manager: &mut EntityComponentManager) {
        println!("System running");
    }
}

fn main() {
    let mut world = World::new();

    world
        .create_entity()
        .with(Size {})
        .with(Background)
        .with(Selector)
        .with(Depth { value: 0 })
        .build();

    world
        .create_entity()
        .with(Size {})
        .with(Background)
        .with(Selector)
        .with(Depth { value: 1 })
        .build();

    world
        .create_entity()
        .with(Size {})
        .with(Background {})
        .with(Selector {})
        .with(Depth { value: 2 })
        .build();

    world
        .create_entity()
        .with(Size {})
        .with(Selector {})
        .with(Depth { value: 3 })
        .build();

    world
        .create_entity()
        .with(Size {})
        .with(Selector {})
        .with(Depth { value: 4 })
        .build();

    world
        .create_system(PrintSystem {})
        .with_priority(1)
        .with_filter(|c| {
            for co in c {
                if let Some(_) = co.downcast_ref::<Background>() {
                    return true;
                }
            }
            false
        })
        .with_sort(|ac, bc| {
            let mut a_detph = 0;
            let mut b_depth = 0;

            if let Some(depth) = ac.downcast_ref::<Depth>() {
                a_detph = depth.value;
            }

            if let Some(depth) = bc.downcast_ref::<Depth>() {
                b_depth = depth.value;
            }

            a_detph.cmp(&b_depth)
        })
        .build();
}
