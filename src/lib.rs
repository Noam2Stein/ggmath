#![feature(const_refs_to_cell)]
#![feature(const_mut_refs)]

mod copy_elements;
mod vec_swizle;
mod vec_op;

mod element;
pub use element::*;

mod gen;
pub use gen::*;