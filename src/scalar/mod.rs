//! Scalars are mathamatical types that have magnitude but no direction.
//! - [f32] and [bool] are scalars.
//! - [Vec3](crate::vec::Vec3) is not a scalar.

mod primitive_impls;

pub mod aliases;

pub use crate::vec::interfaces::Scalar;

pub use ggmath_proc_macros::scalar_aliases;
