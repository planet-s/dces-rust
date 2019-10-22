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
//! ```rust
//! extern crate dces;
//! use dces::prelude::*;
//!
//! #[derive(Default)]
//! struct Name { value: String }
//!
//! struct PrintSystem;
//!
//! impl System<VecEntityStore> for PrintSystem {
//!     fn run(&self, ecm: &mut EntityComponentManager<VecEntityStore>) {
//!         let (e_store, c_store) = ecm.stores();
//!
//!         for entity in &e_store.inner {
//!             if let Ok(comp) = c_store.borrow_component::<Name>(*entity) {
//!                 println!("{}", comp.value);
//!             }
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let mut world = World::<VecEntityStore>::new();
//!
//!     world.create_entity().with(Name { value: String::from("DCES") }).build();
//!     world.create_system(PrintSystem).build();
//!
//!     world.run();
//! }
//! ```
pub mod component;
pub mod error;
pub mod prelude;
pub mod system;
pub mod world;
