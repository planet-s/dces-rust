extern crate dces;

use dces::prelude::*;

pub struct Size {
    width: u32,
    height: u32,
}

pub struct Name {
    value: String,
}

pub struct Depth {
    value: u32,
}

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
                    println!("{} width: {}; height: {}", name.value, size.width, size.height);
                }
            }
        }
    }
}

fn main() {
    let mut world = World::new();

    world
        .create_entity()
        .with(Name {
            value: String::from("Button"),
        }).with(Depth { value: 4 })
        .with(Size {
            width: 5,
            height: 5,
        }).build();

    world
        .create_entity()
        .with(Name {
            value: String::from("CheckBox"),
        }).with(Depth { value: 1 })
        .with(Size {
            width: 3,
            height: 3,
        }).build();

    world
        .create_entity()
        .with(Name {
            value: String::from("RadioButton"),
        }).with(Depth { value: 2 })
        .with(Size {
            width: 4,
            height: 6,
        }).build();

    world
        .create_entity()
        .with(Depth { value: 3 })
        .with(Size {
            width: 10,
            height: 4,
        }).build();

    world
        .create_entity()
        .with(Depth { value: 0 })
        .with(Size {
            width: 5,
            height: 8,
        }).build();

    world.create_system(SizeSystem).with_priority(0).build();

    world
        .create_system(PrintSystem)
        .with_priority(1)

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
            let detph_a;
            let detph_b;

            if let Some(depth) = comp_a.downcast_ref::<Depth>() {
                detph_a = depth.value;
            } else {
                return None;
            }

            if let Some(depth) = comp_b.downcast_ref::<Depth>() {
                detph_b = depth.value;
            } else {
                return None;
            }

            Some(detph_a.cmp(&detph_b))
        }).build();

    world.run();
}
