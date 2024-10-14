//! A default implementation for Scalar that doesn't use SIMD.
//!
//! Use the [scalar_default_impl] macro.

use crate::vec::inner::*;

use super::*;

mod vec;

pub use gomath_proc_macros::scalar_default_impl;

/// Automatically implements Scalar using a default implementation.
/// Has scalar supertraits that can't be implemented automatically by generics.
///
/// Use [scalar_default_impl] instead.
pub trait ScalarDefaultImpl: Construct + PartialOrd + ScalarAlignedVecs {}

impl<T: ScalarDefaultImpl> Scalar for T {}
