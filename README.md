# DCES

DCES is a library that provides a variant of the Entity Component System: https://en.wikipedia.org/wiki/Entity–component–system.

## Features:

* Filter and sort entities for systems
* Define priorities (run order) for systems

Notice: The library is still in beta. API changes are possible.

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

## Example

```rust
 extern crate dces;
 use dces::prelude::*;

 struct Name { value: String }

 struct PrintSystem;

 impl System for PrintSystem {
     fn run(&self, entities: &Vec<Entity>, ecm: &mut EntityComponentManager) {
         for entity in entities {
             if let Ok(comp) = ecm.borrow_component::<Name>(*entity) {
                 println!("{}", comp.value);
             }
         }
     }
 }

 fn main() {
     let mut world = World::new();

     world.create_entity().with(Name { value: String::from("DCES") }).build();
     world.create_system(PrintSystem).build();

     world.run();
 }
```

## Future features

* Apply implicit system filter and sort
* Concurrency of systems with same priority
* Advanced example
* Speed up system entity sort / filter (run over entities -> add to system by filter)

## World

* Developer Interface
* Iterate over ES
* Handles ECS

## ECM: Entity Component Manager (singelton)

* Knows all entities as ids
* Contains vector of all components
* Components referenced by entity ids

## ES: Entity System (0..n)

* Knows filtered subset of entities e.g. render entities for render system
* Provides one system run function
* Read and write to components