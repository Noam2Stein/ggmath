pub mod aliases;
pub mod default_impl;

mod primitive_impls;

use crate::{construct::Construct, vec::ScalarVec};

pub trait Scalar: Construct + ScalarVec {}

pub use gomath_proc_macros::scalar_aliases;
