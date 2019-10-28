#![crate_name = "dces"]
#![crate_type = "lib"]
#![deny(warnings)]

//! # DCES
//!
//! DCES is a library that provides a variant of the entity component system.
//!
//! Features:
//!
//! * Filter and sort entities for systems
//! * Define priorities (run order) for systems
//!
//! **The library is still WIP. API changes are possible.**
//!
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//!
//! # Example
//!
//! ```no_run
//! use dces::prelude::*;
//!
//! #[derive(Default)]
//! struct Name {
//!    value: String,
//! }
//!
//! struct PrintSystem;
//!
//! impl System<EntityStore, ComponentStore> for PrintSystem {
//!    fn run(&self, ecm: &mut EntityComponentManager<EntityStore, ComponentStore>) {
//!        let (e_store, c_store) = ecm.stores();
//!
//!        for entity in &e_store.inner {
//!            if let Ok(comp) = c_store.get::<Name>(*entity) {
//!                println!("{}", comp.value);
//!            }
//!        }
//!    }
//! }
//!
//!
//! let mut world = World::from_stores(EntityStore::default(), ComponentStore::default());
//!
//! world
//!     .create_entity()
//!     .components(
//!         ComponentBuilder::new()
//!             .with(Name {
//!                 value: String::from("DCES"),
//!             })
//!             .build(),
//!     )
//!     .build();
//!
//! world.create_system(PrintSystem).build();
//! world.run();
//!
//!
//! ```
pub mod component;
pub mod entity;
pub mod error;
pub mod prelude;
pub mod system;
pub mod world;
