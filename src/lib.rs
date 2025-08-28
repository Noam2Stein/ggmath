use std::panic::{RefUnwindSafe, UnwindSafe};

#[cfg(feature = "vector")]
pub mod vector;

pub trait Construct: Sized + Send + Sync + 'static + Copy + UnwindSafe + RefUnwindSafe {}

pub struct Usize<const N: usize>;

impl<T: Sized + Send + Sync + 'static + Copy + UnwindSafe + RefUnwindSafe> Construct for T {}
