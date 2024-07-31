#![feature(const_refs_to_cell)]
#![feature(const_mut_refs)]

mod copy;
mod swizzle;

mod element;
pub use element::*;

mod gen;
pub use gen::*;