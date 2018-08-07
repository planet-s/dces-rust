extern crate dces;

use dces::prelude::*;

struct Size {
    width: u32,
    height: u32,
}

struct Name(String);

struct Depth(u32);

pub struct SizeSystem;
impl System for SizeSystem {
    fn run(&self, entities: &Vec<Entity>, ecm: &mut EntityComponentManager) {
        for entity in entities {
            if let Ok(comp) = ecm.borrow_mut_component::<Size>(*entity) {
                comp.width += 1;
                comp.height += 1;
            }
        }
    }
}

pub struct PrintSystem;
impl System for PrintSystem {
    fn run(&self, entities: &Vec<Entity>, ecm: &mut EntityComponentManager) {
        for entity in entities {
            if let Ok(name) = ecm.borrow_component::<Name>(*entity) {
                if let Ok(size) = ecm.borrow_component::<Size>(*entity) {
                    println!("{} width: {}; height: {}", name.0, size.width, size.height);
                }
            }
        }
    }
}

fn main() {
    let mut world = World::new();

    world
        .create_entity()
        .with(Name(String::from("Button"))).with(Depth(4))
        .with(Size {
            width: 5,
            height: 5,
        }).build();

    world
        .create_entity()
        .with(Name (String::from("CheckBox"),)).with(Depth(1))
        .with(Size {
            width: 3,
            height: 3,
        }).build();

    world
        .create_entity()
        .with(Name (String::from("RadioButton"))).with(Depth(2))
        .with(Size {
            width: 4,
            height: 6,
        }).build();

    world
        .create_entity()
        .with(Depth(3))
        .with(Size {
            width: 10,
            height: 4,
        }).build();

    world
        .create_entity()
        .with(Depth(0))
        .with(Size {
            width: 5,
            height: 8,
        }).build();

    world
        .create_system(PrintSystem)
        .with_priority(&1)

        // filter entities with Name components
        .with_filter(|comp| {
            for co in comp {
                if let Some(_) = co.downcast_ref::<Name>() {
                    return true;
                }
            }
            false
        })
        
        // sort entities by depth
        .with_sort(|comp_a, comp_b| {
            let depth_a;
            let depth_b;

            if let Some(depth) = comp_a.downcast_ref::<Depth>() {
                depth_a = depth;
            } else {
                return None;
            }

            if let Some(depth) = comp_b.downcast_ref::<Depth>() {
                depth_b = depth;
            } else {
                return None;
            }

            Some(depth_a.0.cmp(&depth_b.0))
        }).build();

        world.create_system(SizeSystem).with_priority(&0).build();

    world.apply_filter_and_sort();
    world.run();
}
