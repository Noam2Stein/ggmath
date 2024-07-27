#![feature(const_refs_to_cell)]
#![feature(const_mut_refs)]

mod copy_elements;
mod swizzle_fns;

mod element;
pub use element::*;

mod gen;
pub use gen::*;