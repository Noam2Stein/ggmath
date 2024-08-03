#![feature(const_refs_to_cell)]
#![feature(const_mut_refs)]

mod copy;
mod cast;

mod element;
pub use element::*;

mod op;
mod swizzle;

mod gen;
pub use gen::*;