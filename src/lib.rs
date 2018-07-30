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
//! Notice: The library is still in beta. API changes are possible.
//!
//! # Example
//!
//! ```rust
//! extern crate dces;
//! use dces::prelude::*;
//!
//! struct Name { value: String }
//!
//! struct PrintSystem;
//!
//! impl System for PrintSystem {
//!     fn run(&self, entities: &Vec<Entity>, ecm: &mut EntityComponentManager) {
//!         for entity in entities {
//!             if let Ok(comp) = ecm.borrow_component::<Name>(*entity) {
//!                 println!("{}", comp.value);
//!             }
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let mut world = World::new();
//!
//!     world.create_entity().with(Name { value: String::from("DCES") }).build();
//!     world.create_system(PrintSystem).build();
//!
//!     world.run();
//! }
//! ```
pub use self::entity::{Component, Entity, EntityBuilder, EntityComponentManager};
pub use self::error::NotFound;
pub use self::system::{EntitySystem, EntitySystemBuilder, EntitySystemManager, System};
pub use self::world::World;

pub mod entity;
pub mod error;
pub mod prelude;
pub mod system;
pub mod world;
