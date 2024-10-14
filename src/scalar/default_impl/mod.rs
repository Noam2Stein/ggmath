use crate::vec::ScalarAlignedVecs;

use super::*;

mod vec;

pub use gomath_proc_macros::scalar_default_impl;

pub trait ScalarDefaultImpl: Construct + ScalarAlignedVecs {}

impl<T: ScalarDefaultImpl> Scalar for T {}
