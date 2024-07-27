#![feature(const_refs_to_cell)]
#![feature(const_mut_refs)]

mod copy_components;

mod component;
pub use component::*;

mod gen;
pub use gen::*;