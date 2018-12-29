# DCES

DCES is a library that provides a variant of the Entity Component System: https://en.wikipedia.org/wiki/Entity–component–system.

The goal of DCES is a lightweight ECS library with zero dependencies used by UI frameworks and game engines. It is being developed as part of OrbTk an (G)UI framework written in Rust. All widgets and properties of OrbTk are handled by DCES. 

[![Build status](https://gitlab.redox-os.org/redox-os/dces-rust/badges/master/build.svg)](https://gitlab.redox-os.org/redox-os/dces-rust/pipelines)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-v0.1.1-orange.svg)](https://crates.io/crates/dces)
[![docs.rs](https://docs.rs/dces/badge.svg)](https://docs.rs/dces)

## Features:

* Register entities with components
* Share components between entities
* Register systems and read / write components of entities
* Order systems execution by priority
* Register container for entity organisation (Vec, HashMap, Custom Container, ...)

## Usage

To include DCES in your project, just add the dependency
line to your `Cargo.toml` file:

```text
dces = "0.1.2"
```

To use DCES master, just add the dependency
line to your `Cargo.toml` file:

```text
dces = { git = https://gitlab.redox-os.org/redox-os/dces-rust.git }
```

## Example

```rust
extern crate dces;
use dces::prelude::*;

struct Name { value: String }

struct PrintSystem;

impl System<VecEntityContainer> for PrintSystem {
    fn run(&self, entities: &VecEntityContainer, ecm: &mut EntityComponentManager) {
        for entity in &entities.inner {
            if let Ok(comp) = ecm.borrow_component::<Name>(*entity) {
                println!("{}", comp.value);
            }
        }
    }
}

fn main() {
    let mut world = World::<VecEntityContainer>::new();

    world.create_entity().with(Name { value: String::from("DCES") }).build();
    world.create_system(PrintSystem).build();

    world.run();
}
```

You could find additional examples in the `examples/` directory.

You can start the `basic` example by executing the following command:

```text
cargo run --example basic
```

## Build and run documenation

You can build and run the latest documentation by executing the following command:

```text
cargo doc --no-deps --open
```

## Future features

* Concurrency of systems with same priority
* Advanced example
* Book

## Inspirations

* [Specs - Parallel ECS](https://github.com/slide-rs/specs)
* [Rustic Entity-Component System](https://github.com/AndyBarron/rustic-ecs)

## FAQ

### Why not Specs
Because DCES is developed to fulfill the requirements of OrbTk. To reduce the dependency tree of OrbTk
DCES depends on zero crates.