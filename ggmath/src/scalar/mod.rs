//! Scalars are mathamatical types that have magnitude but no direction.
//! - [f32] and [bool] are scalars.
//! - [Vec3](crate::vec::Vec3) is not a scalar.

mod primitive_impls;

pub use crate::vector::interfaces::scalar_traits::*;

pub use ggmath_proc_macros::inner_vectors;
