#[macro_use] extern crate enum_primitive;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate direction;
extern crate cgmath;
#[macro_use] extern crate entity_store_helper;
extern crate append;

pub mod entity_store {
    include_entity_store!("entity_store.rs");
}

mod policy;
mod prototypes;

pub mod input;
pub mod state;
pub mod tile;
