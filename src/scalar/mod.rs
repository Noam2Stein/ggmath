//! Scalars are mathamatical types that have magnitude but no direction.
//! - [f32] and [bool] are scalars.
//! - [Vec3](crate::vec::Vec3) is not a scalar.

pub mod aliases;
pub mod default_impl;

mod primitive_impls;

use crate::{construct::Construct, vec::ScalarVec};

/// trait for types that can be put inside mathamatical types like [vectors](crate::vec::Vector) and [matricies](crate::mat::Matrix).
///
/// useful when using mathamatical types while being generic over the scalar type.
/// # Examples
/// ```
/// fn print_x<T: Scalar>(vec: Vec2<T>) {
///     println!("x is equal to {}", vec.x())
/// }
/// ```
///
/// # Implementing [Scalar]
/// To implement [Scalar] you need to implement all [Vector](crate::vec::Vector) fns for the scalar type.
/// This is so that each vector fn can be optimized differently for each scalar.
/// for example, [f32] uses SIMD to implement fns on most targets.
///
/// To make an unoptimized scalar type use [scalar_default_impl](default_impl::scalar_default_impl).
///
/// To make a wrapper scaler type for an existing scalar (for example Meters(f32)) use ```todo!()```
pub trait Scalar: Construct + PartialOrd + ScalarVec {}

pub use gomath_proc_macros::scalar_aliases;
