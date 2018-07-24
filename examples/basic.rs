extern crate dces;

use dces::prelude::*;

pub struct Size;



fn main() {
    let mut world = World::new();

    let entity = world.create_entity().with(Size {}).build();

    println!("Entity: {}", entity);
}