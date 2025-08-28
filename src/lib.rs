use std::panic::{RefUnwindSafe, UnwindSafe};

mod generated;
#[allow(unused_imports)]
pub use generated::*;

mod primitives;
#[allow(unused_imports)]
pub use primitives::*;

#[cfg(feature = "vector")]
pub mod vector;

pub trait Construct: Sized + Send + Sync + 'static + Copy + UnwindSafe + RefUnwindSafe {}

pub struct Usize<const N: usize>;

impl<T: Sized + Send + Sync + 'static + Copy + UnwindSafe + RefUnwindSafe> Construct for T {}
